use crate::{application::APP, prelude::*};
use aws_sdk_kms::Client;
use bip32::PrivateKey;
use cosmos_gravity::crypto::{CosmosSigner, EthPubkey, DEFAULT_HD_PATH};
use ethers::providers::Middleware;
use ethers::{
    signers::{LocalWallet as EthWallet, Signer},
    types::Chain,
};
use gravity_utils::{connection_prep::create_rpc_connections, ethereum::downcast_to_u64};
use pkcs8::LineEnding;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use signatory::FsKeyStore;
use std::net::SocketAddr;
use std::path::Path;
use std::{io, time::Duration};

use crate::utils::aws::{AwsSigner, AwsSignerError, WrapperSigner};

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str)]
pub enum Keystore {
    Aws,
    #[serde(other)]
    File(String),
}

impl Default for Keystore {
    fn default() -> Self {
        Keystore::File("/tmp/keystore".to_owned())
    }
}

async fn get_client() -> Client {
    let shared_config = aws_config::load_from_env().await;
    Client::new(&shared_config)
}

async fn delete_secret(secret_id: String) -> Result<(), aws_sdk_kms::Error> {
    let client = get_client().await;
    let req = client.schedule_key_deletion().key_id(secret_id);
    let _ = req.send().await?;
    Ok(())
}

async fn describe_secret(secret_id: String) -> Result<signatory::KeyInfo, aws_sdk_kms::Error> {
    let client = get_client().await;
    let e = aws_sdk_kms::Error::Unhandled(Box::<io::Error>::new(io::ErrorKind::Other.into()));
    let req = client.describe_key().key_id(secret_id);
    let r = req.send().await?;
    if let Some(Some(key_id)) = r.key_metadata().map(|x| x.key_id()) {
        Ok(signatory::KeyInfo {
            name: signatory::KeyName::new(key_id).map_err(|_| e)?,
            algorithm: None,
            encrypted: false,
        })
    } else {
        Err(e)
    }
}

async fn get_aws_kms_signer(secret_id: String) -> Result<WrapperSigner, AwsSignerError> {
    let client = get_client().await;
    let config = APP.config();
    let connections = create_rpc_connections(
        "".to_string(),
        None,
        Some(config.ethereum.rpc.clone()),
        Duration::from_secs(3),
    )
    .await;
    let provider = connections.eth_provider.clone().unwrap();
    let chain_id = provider
        .get_chainid()
        .await
        .unwrap_or_else(|_| Chain::Mainnet.into());
    let chain_id = downcast_to_u64(chain_id).expect("Chain ID overflowed when downcasting to u64");

    Ok(WrapperSigner::Aws(
        AwsSigner::new(client, secret_id, chain_id).await?,
    ))
}

impl Keystore {
    /// Load a PKCS#8 key from the keystore.
    pub fn load(&self, name: &signatory::KeyName) -> signatory::Result<pkcs8::SecretDocument> {
        match self {
            Keystore::File(path) => {
                let keystore = Path::new(path);
                let keystore = FsKeyStore::create_or_open(keystore)?;
                keystore.load(name)
            }
            Keystore::Aws => Err(signatory::Error::Io(io::Error::new(
                io::ErrorKind::Other,
                "Loading secrets is not supported on AWS KMS".to_owned(),
            ))),
        }
    }
    /// Get information about a key with the given name.
    pub fn info(&self, name: &signatory::KeyName) -> signatory::Result<signatory::KeyInfo> {
        match self {
            Keystore::File(path) => {
                let keystore = Path::new(path);
                let keystore = FsKeyStore::create_or_open(keystore)?;
                keystore.info(name)
            }
            Keystore::Aws => {
                let rt = tokio::runtime::Runtime::new()?;
                let info = rt.block_on(describe_secret(name.to_string()));
                info.map_err(|e| signatory::Error::Io(io::Error::new(io::ErrorKind::Other, e)))
            }
        }
    }

    /// Import a PKCS#8 key into the keystore.
    pub fn store(
        &self,
        name: &signatory::KeyName,
        der: &pkcs8::der::SecretDocument,
    ) -> signatory::Result<()> {
        match self {
            Keystore::File(path) => {
                let keystore = Path::new(path);
                let keystore = FsKeyStore::create_or_open(keystore)?;
                keystore.store(name, der)
            }
            Keystore::Aws => Err(signatory::Error::Io(io::Error::new(
                io::ErrorKind::Other,
                "Storing secrets is not supported for asymmetric key materials on AWS KMS"
                    .to_owned(),
            ))),
        }
    }

    /// Delete a PKCS#8 key from the keystore.
    pub fn delete(&self, name: &signatory::KeyName) -> signatory::Result<()> {
        match self {
            Keystore::File(path) => {
                let keystore = Path::new(path);
                let keystore = FsKeyStore::create_or_open(keystore)?;
                keystore.delete(name)
            }
            Keystore::Aws => {
                let rt = tokio::runtime::Runtime::new()?;

                rt.block_on(delete_secret(name.to_string()))
                    .map_err(|e| signatory::Error::Io(io::Error::new(io::ErrorKind::Other, e)))
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GorcConfig {
    pub keystore: Keystore,
    pub gravity: GravitySection,
    pub ethereum: EthereumSection,
    pub cosmos: CosmosSection,
    pub metrics: MetricsSection,
}

impl GorcConfig {
    fn load_secret_key(&self, name: String) -> k256::elliptic_curve::SecretKey<k256::Secp256k1> {
        let name = name.parse().expect("Could not parse name");
        let key = self.keystore.load(&name).expect("Could not load key");
        key.to_pem("PRIVATE KEY", LineEnding::LF)
            .expect("encode")
            .parse()
            .expect("Could not parse pem")
    }

    pub fn load_ethers_wallet(&self, name: String) -> impl Signer + Clone + EthPubkey {
        if matches!(self.keystore, Keystore::Aws) {
            let rt = tokio::runtime::Runtime::new().expect("cannot get Tokio runtime");

            rt.block_on(get_aws_kms_signer(name))
                .expect("Could not get AWS KMS signer")
        } else {
            WrapperSigner::Local(EthWallet::from(self.load_secret_key(name)))
        }
    }

    pub fn load_deep_space_key(&self, name: String) -> impl CosmosSigner {
        if matches!(self.keystore, Keystore::Aws) {
            let rt = tokio::runtime::Runtime::new().expect("cannot get Tokio runtime");

            rt.block_on(get_aws_kms_signer(name))
                .expect("Could not get AWS KMS signer")
        } else {
            let key = self.load_secret_key(name).to_bytes();
            let key = deep_space::utils::bytes_to_hex_str(&key);
            let pk: cosmos_gravity::crypto::PrivateKey =
                key.parse().expect("Could not parse private key");
            WrapperSigner::LocalCosmos(pk)
        }
    }
}

impl Default for GorcConfig {
    fn default() -> Self {
        Self {
            keystore: Keystore::default(),
            gravity: GravitySection::default(),
            ethereum: EthereumSection::default(),
            cosmos: CosmosSection::default(),
            metrics: MetricsSection::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GravitySection {
    pub contract: String,
    pub payment_address: String,
    pub fees_denom: String,
}

impl Default for GravitySection {
    fn default() -> Self {
        Self {
            contract: "0x0000000000000000000000000000000000000000".to_owned(),
            payment_address: "0x0000000000000000000000000000000000000000".to_owned(),
            fees_denom: "stake".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct EthereumSection {
    pub key_derivation_path: String,
    pub rpc: String,
    pub gas_price_multiplier: f32,
    pub gas_multiplier: f32,
    pub blocks_to_search: u64,
}

impl Default for EthereumSection {
    fn default() -> Self {
        Self {
            key_derivation_path: "m/44'/60'/0'/0/0".to_owned(),
            rpc: "http://localhost:8545".to_owned(),
            gas_price_multiplier: 1.0f32,
            gas_multiplier: 1.0f32,
            blocks_to_search: 5000,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CosmosSection {
    pub key_derivation_path: String,
    pub grpc: String,
    pub prefix: String,
    pub gas_adjustment: f64,
    pub msg_batch_size: u32,
    pub gas_price: GasPrice,
    pub granter: Option<String>,
}

impl Default for CosmosSection {
    fn default() -> Self {
        Self {
            key_derivation_path: DEFAULT_HD_PATH.to_owned(),
            grpc: "http://localhost:9090".to_owned(),
            prefix: "cosmos".to_owned(),
            gas_price: GasPrice::default(),
            gas_adjustment: 1.0f64,
            msg_batch_size: 5,
            granter: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GasPrice {
    pub amount: f64,
    pub denom: String,
}

impl Default for GasPrice {
    fn default() -> Self {
        Self {
            amount: 0.001,
            denom: "stake".to_owned(),
        }
    }
}

impl GasPrice {
    pub fn as_tuple(&self) -> (f64, String) {
        (self.amount, self.denom.to_owned())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct MetricsSection {
    pub listen_addr: SocketAddr,
}

impl Default for MetricsSection {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:3000".parse().unwrap(),
        }
    }
}
