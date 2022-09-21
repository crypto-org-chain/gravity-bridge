///! Adapted from this PR: https://github.com/gakonst/ethers-rs/pull/1628
///! The previous code this is based on is dual-licensed under MIT/Apache 2: Copyright (c) 2020 Georgios Konstantopoulos
use aws_sdk_kms::{
    error::{GetPublicKeyError, SignError},
    model::{MessageType, SigningAlgorithmSpec},
    output::{GetPublicKeyOutput, SignOutput},
    types::{Blob, SdkError},
    Client as KmsClient,
};
use cosmos_gravity::crypto::{CosmosSigner, EthPubkey, PrivateKey};
use deep_space::{
    private_key::{SignType, TxParts},
    utils::encode_any,
    COSMOS_PUBKEY_URL,
};
use ethers::{
    core::{
        k256::ecdsa::{Error as K256Error, Signature as KSig, VerifyingKey},
        types::{
            transaction::{eip2718::TypedTransaction, eip712::Eip712},
            Address, Signature as EthSig, H256,
        },
        utils::hash_message,
    },
    signers::{LocalWallet, Signer, WalletError},
    utils::hex::ToHex,
};
use gravity_proto::cosmos_sdk_proto::cosmos::{
    crypto::secp256k1::PubKey as ProtoSecp256k1Pubkey,
    tx::v1beta1::{mode_info, AuthInfo, ModeInfo, SignDoc, SignerInfo, TxBody, TxRaw},
};
use gravity_utils::{error::GravityError, ethereum::bytes_to_hex_str};
use k256::sha2::{Digest, Sha256};
use prost::Message;
use tracing::{debug, instrument, trace};

use ethers::core::{
    k256::{
        ecdsa::recoverable::{Id, Signature as RSig},
        elliptic_curve::sec1::ToEncodedPoint,
        FieldBytes,
    },
    types::U256,
    utils::keccak256,
};

/// A workaround to be able to return a single type for the signer traits
#[derive(Clone, Debug)]
pub enum WrapperSigner {
    Aws(AwsSigner),
    Local(LocalWallet),
    LocalCosmos(PrivateKey),
}

/// Errors produced by the AwsSigner
#[allow(clippy::large_enum_variant)]
#[derive(thiserror::Error, Debug)]
pub enum WrapperSignerError {
    #[error("{0}")]
    Aws(AwsSignerError),
    #[error("{0}")]
    Local(WalletError),
    #[error("unsupported wrapper signer")]
    Unsupported,
}

impl From<AwsSignerError> for WrapperSignerError {
    fn from(e: AwsSignerError) -> Self {
        WrapperSignerError::Aws(e)
    }
}

impl From<WalletError> for WrapperSignerError {
    fn from(e: WalletError) -> Self {
        WrapperSignerError::Local(e)
    }
}

#[async_trait::async_trait]
impl Signer for WrapperSigner {
    type Error = WrapperSignerError;
    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<EthSig, Self::Error> {
        match self {
            WrapperSigner::Aws(signer) => {
                let r = signer.sign_message(message).await?;
                Ok(r)
            }
            WrapperSigner::Local(signer) => {
                let r = signer.sign_message(message).await?;
                Ok(r)
            }
            _ => Err(WrapperSignerError::Unsupported),
        }
    }

    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<EthSig, Self::Error> {
        match self {
            WrapperSigner::Aws(signer) => {
                let r = signer.sign_transaction(tx).await?;
                Ok(r)
            }
            WrapperSigner::Local(signer) => {
                let r = signer.sign_transaction(tx).await?;
                Ok(r)
            }
            _ => Err(WrapperSignerError::Unsupported),
        }
    }

    async fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<EthSig, Self::Error> {
        match self {
            WrapperSigner::Aws(signer) => {
                let r = signer.sign_typed_data(payload).await?;
                Ok(r)
            }
            WrapperSigner::Local(signer) => {
                let r = signer.sign_typed_data(payload).await?;
                Ok(r)
            }
            _ => Err(WrapperSignerError::Unsupported),
        }
    }

    fn address(&self) -> Address {
        match self {
            WrapperSigner::Aws(signer) => signer.address(),
            WrapperSigner::Local(signer) => signer.address(),
            _ => unreachable!("local Cosmos Key used for Ethereum"),
        }
    }

    /// Returns the signer's chain id
    fn chain_id(&self) -> u64 {
        match self {
            WrapperSigner::Aws(signer) => signer.chain_id(),
            WrapperSigner::Local(signer) => signer.chain_id(),
            _ => unreachable!("unsupported signer (local Cosmos Key used for Ethereum)"),
        }
    }

    /// Sets the signer's chain id
    fn with_chain_id<T: Into<u64>>(self, chain_id: T) -> Self {
        match self {
            WrapperSigner::Aws(signer) => WrapperSigner::Aws(signer.with_chain_id(chain_id)),
            WrapperSigner::Local(signer) => WrapperSigner::Local(signer.with_chain_id(chain_id)),
            _ => unreachable!("unsupported signer (local Cosmos Key used for Ethereum)"),
        }
    }
}

impl EthPubkey for WrapperSigner {
    fn to_public_key(&self) -> VerifyingKey {
        match self {
            WrapperSigner::Aws(signer) => signer.to_public_key(),
            WrapperSigner::Local(signer) => signer.to_public_key(),
            _ => unreachable!("unsupported signer (local Cosmos Key used for Ethereum)"),
        }
    }
}

#[async_trait::async_trait]
impl CosmosSigner for WrapperSigner {
    fn to_address(&self, prefix: &str) -> Result<deep_space::Address, GravityError> {
        match self {
            WrapperSigner::Aws(signer) => signer.to_address(prefix),
            WrapperSigner::LocalCosmos(signer) => signer
                .to_address(prefix)
                .map_err(GravityError::CosmosPrivateKeyError),
            _ => Err(GravityError::CosmosSignerError(Box::new(
                WrapperSignerError::Unsupported,
            ))),
        }
    }
    async fn sign_std_msg(
        &self,
        messages: &[deep_space::Msg],
        args: deep_space::MessageArgs,
        memo: String,
    ) -> Result<Vec<u8>, GravityError> {
        match self {
            WrapperSigner::Aws(signer) => signer.sign_std_msg(messages, args, memo),
            WrapperSigner::LocalCosmos(signer) => signer.sign_std_msg(messages, args, memo),
            _ => {
                return Err(GravityError::CosmosSignerError(Box::new(
                    WrapperSignerError::Unsupported,
                )));
            }
        }
        .await
    }
    async fn build_tx(
        &self,
        messages: &[deep_space::Msg],
        args: deep_space::MessageArgs,
        memo: String,
    ) -> Result<TxParts, GravityError> {
        match self {
            WrapperSigner::Aws(signer) => signer.build_tx(messages, args, memo),
            WrapperSigner::LocalCosmos(signer) => signer.build_tx(messages, args, memo),
            _ => {
                return Err(GravityError::CosmosSignerError(Box::new(
                    WrapperSignerError::Unsupported,
                )));
            }
        }
        .await
    }
}

/// Converts a recoverable signature to an ethers signature
fn rsig_to_ethsig(sig: &RSig) -> EthSig {
    let v: u8 = sig.recovery_id().into();
    let v = (v + 27) as u64;
    let r_bytes: FieldBytes = sig.r().into();
    let s_bytes: FieldBytes = sig.s().into();
    let r = U256::from_big_endian(r_bytes.as_slice());
    let s = U256::from_big_endian(s_bytes.as_slice());
    EthSig { r, s, v }
}

/// Makes a trial recovery to check whether an RSig corresponds to a known
/// `VerifyingKey`
fn check_candidate(sig: &RSig, digest: [u8; 32], vk: &VerifyingKey) -> bool {
    if let Ok(key) = sig.recover_verifying_key_from_digest_bytes(digest.as_ref().into()) {
        key == *vk
    } else {
        false
    }
}

/// Recover an rsig from a signature under a known key by trial/error
fn rsig_from_digest_bytes_trial_recovery(
    sig: &KSig,
    digest: [u8; 32],
    vk: &VerifyingKey,
) -> Result<RSig, AwsSignerError> {
    let err = |_| AwsSignerError::Other("Bad signature".to_string());
    let sig_0 = RSig::new(sig, Id::new(0).expect("correct recovery id")).map_err(err)?;
    let sig_1 = RSig::new(sig, Id::new(1).expect("correct recovery id")).map_err(err)?;

    if check_candidate(&sig_0, digest, vk) {
        Ok(sig_0)
    } else if check_candidate(&sig_1, digest, vk) {
        Ok(sig_1)
    } else {
        Err(AwsSignerError::Other("Bad signature".to_string()))
    }
}

/// Modify the v value of a signature to conform to eip155
fn apply_eip155(sig: &mut EthSig, chain_id: u64) {
    let v = (chain_id * 2 + 35) + ((sig.v - 1) % 2);
    sig.v = v;
}

/// Convert a verifying key to an ethereum address
pub fn verifying_key_to_address(key: &VerifyingKey) -> Address {
    // false for uncompressed
    let uncompressed_pub_key = key.to_encoded_point(false);
    let public_key = uncompressed_pub_key.to_bytes();
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);
    Address::from_slice(&hash[12..])
}

/// Decode an AWS KMS Pubkey response
fn decode_pubkey(resp: GetPublicKeyOutput) -> Result<VerifyingKey, AwsSignerError> {
    let raw = resp
        .public_key
        .ok_or_else(|| AwsSignerError::from("Pubkey not found in response".to_owned()))?;

    let spk = spki::SubjectPublicKeyInfo::try_from(raw.as_ref())?;
    let key = VerifyingKey::from_sec1_bytes(spk.subject_public_key)?;

    Ok(key)
}

/// Decode an AWS KMS Signature response
fn decode_signature(resp: SignOutput) -> Result<KSig, AwsSignerError> {
    let raw = resp
        .signature
        .ok_or_else(|| AwsSignerError::from("Signature not found in response".to_owned()))?;

    let sig = KSig::from_der(raw.as_ref())?;
    Ok(sig.normalize_s().unwrap_or(sig))
}

/// An ethers Signer that uses keys held in Amazon AWS KMS.
///
/// The AWS Signer passes signing requests to the cloud service. AWS KMS keys
/// are identified by a UUID, the `key_id`.
///
/// Because the public key is unknown, we retrieve it on instantiation of the
/// signer. This means that the new function is `async` and must be called
/// within some runtime.
///
/// ```compile_fail
/// use aws_config::meta::region::RegionProviderChain;
/// use aws_sdk_kms::{Client as KmsClient};
///
/// use ethers_signers::Signer;
/// let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
/// let config = aws_config::from_env().region(region_provider).load().await;
/// let kms_client = KmsClient::new(config);
/// let key_id = "...";
/// let chain_id = 1;
///
/// let signer = AwsSigner::new(kms_client, key_id, chain_id).await?;
/// let sig = signer.sign_message(H256::zero()).await?;
/// ```
#[derive(Clone)]
pub struct AwsSigner {
    kms: KmsClient,
    chain_id: u64,
    key_id: String,
    pubkey: VerifyingKey,
    address: Address,
}

impl EthPubkey for AwsSigner {
    fn to_public_key(&self) -> VerifyingKey {
        self.pubkey
    }
}

impl std::fmt::Debug for AwsSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pubkey_hex: String = self.pubkey.to_bytes().encode_hex();
        f.debug_struct("AwsSigner")
            .field("key_id", &self.key_id)
            .field("chain_id", &self.chain_id)
            .field("pubkey", &pubkey_hex)
            .field("address", &self.address)
            .finish()
    }
}

impl std::fmt::Display for AwsSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AwsSigner {{ address: {}, chain_id: {}, key_id: {} }}",
            self.address, self.chain_id, self.key_id
        )
    }
}

/// Errors produced by the AwsSigner
#[derive(thiserror::Error, Debug)]
pub enum AwsSignerError {
    #[error("{0}")]
    SignError(#[from] SdkError<SignError>),
    #[error("{0}")]
    GetPublicKeyError(#[from] SdkError<GetPublicKeyError>),
    #[error("{0}")]
    K256(#[from] K256Error),
    #[error("{0}")]
    Spki(spki::Error),
    #[error("{0}")]
    Other(String),
    /// Error type from Eip712Error message
    #[error("error encoding eip712 struct: {0:?}")]
    Eip712Error(String),
}

impl From<String> for AwsSignerError {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<spki::Error> for AwsSignerError {
    fn from(e: spki::Error) -> Self {
        Self::Spki(e)
    }
}

#[instrument(err, skip(kms, key_id), fields(key_id = %key_id.as_ref()))]
async fn request_get_pubkey<T>(
    kms: &KmsClient,
    key_id: T,
) -> Result<GetPublicKeyOutput, SdkError<GetPublicKeyError>>
where
    T: AsRef<str>,
{
    debug!("Dispatching get_public_key");
    let resp = kms
        .get_public_key()
        .key_id(key_id.as_ref().to_owned())
        .send()
        .await?;
    trace!("{:?}", &resp);
    Ok(resp)
}

#[instrument(err, skip(kms, digest, key_id), fields(digest = %hex::encode(&digest), key_id = %key_id.as_ref()))]
async fn request_sign_digest<T>(
    kms: &KmsClient,
    key_id: T,
    digest: [u8; 32],
) -> Result<SignOutput, SdkError<SignError>>
where
    T: AsRef<str>,
{
    debug!("Dispatching sign");
    let blob = Blob::new(digest);
    let resp = kms
        .sign()
        .key_id(key_id.as_ref().to_owned())
        .message(blob)
        .message_type(MessageType::Digest)
        .signing_algorithm(SigningAlgorithmSpec::EcdsaSha256)
        .send()
        .await?;
    trace!("{:?}", &resp);
    Ok(resp)
}

impl AwsSigner {
    /// Instantiate a new signer from an existing `KmsClient` and Key ID.
    ///
    /// This function retrieves the public key from AWS and calculates the
    /// Etheruem address. It is therefore `async`.
    #[instrument(err, skip(kms, key_id, chain_id), fields(key_id = %key_id.as_ref()))]
    pub async fn new<T>(
        kms: KmsClient,
        key_id: T,
        chain_id: u64,
    ) -> Result<AwsSigner, AwsSignerError>
    where
        T: AsRef<str>,
    {
        let pubkey = request_get_pubkey(&kms, &key_id)
            .await
            .map(decode_pubkey)??;
        let address = verifying_key_to_address(&pubkey);
        let pubkey_hex: String = pubkey.to_bytes().encode_hex();
        debug!(
            "Instantiated AWS signer with pubkey 0x{} and address 0x{:?}",
            pubkey_hex, &address
        );

        Ok(Self {
            kms,
            chain_id,
            key_id: key_id.as_ref().to_owned(),
            pubkey,
            address,
        })
    }

    /// Sign a digest with the key associated with a key id
    pub async fn sign_digest_with_key<T>(
        &self,
        key_id: T,
        digest: [u8; 32],
    ) -> Result<KSig, AwsSignerError>
    where
        T: AsRef<str>,
    {
        request_sign_digest(&self.kms, key_id, digest)
            .await
            .map(decode_signature)?
    }

    /// Sign a digest with this signer's key
    pub async fn sign_digest(&self, digest: [u8; 32]) -> Result<KSig, AwsSignerError> {
        self.sign_digest_with_key(self.key_id.clone(), digest).await
    }

    /// Sign a digest with this signer's key and add the eip155 `v` value
    /// corresponding to the input chain_id
    #[instrument(err, skip(digest), fields(digest = %hex::encode(&digest)))]
    async fn sign_digest_with_eip155(
        &self,
        digest: H256,
        chain_id: u64,
    ) -> Result<EthSig, AwsSignerError> {
        let sig = self.sign_digest(digest.into()).await?;

        let sig = rsig_from_digest_bytes_trial_recovery(&sig, digest.into(), &self.pubkey)?;

        let mut sig = rsig_to_ethsig(&sig);
        apply_eip155(&mut sig, chain_id);
        Ok(sig)
    }
}

#[async_trait::async_trait]
impl Signer for AwsSigner {
    type Error = AwsSignerError;

    #[instrument(err, skip(message))]
    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<EthSig, Self::Error> {
        let message = message.as_ref();
        let message_hash = hash_message(message);
        trace!("{:?}", message_hash);
        trace!("{:?}", message);

        self.sign_digest_with_eip155(message_hash, self.chain_id)
            .await
    }

    #[instrument(err)]
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<EthSig, Self::Error> {
        let mut tx_with_chain = tx.clone();
        let chain_id = tx_with_chain
            .chain_id()
            .map(|id| id.as_u64())
            .unwrap_or(self.chain_id);
        tx_with_chain.set_chain_id(chain_id);

        let sighash = tx.sighash();
        self.sign_digest_with_eip155(sighash, chain_id).await
    }

    async fn sign_typed_data<T: Eip712 + Send + Sync>(
        &self,
        payload: &T,
    ) -> Result<EthSig, Self::Error> {
        let digest = payload
            .encode_eip712()
            .map_err(|e| Self::Error::Eip712Error(e.to_string()))?;

        let sig = self.sign_digest(digest).await?;
        let sig = rsig_from_digest_bytes_trial_recovery(&sig, digest, &self.pubkey)?;
        let sig = rsig_to_ethsig(&sig);

        Ok(sig)
    }

    fn address(&self) -> Address {
        self.address
    }

    /// Returns the signer's chain id
    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Sets the signer's chain id
    fn with_chain_id<T: Into<u64>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }
}

#[async_trait::async_trait]
impl CosmosSigner for AwsSigner {
    fn to_address(&self, prefix: &str) -> Result<deep_space::Address, GravityError> {
        #[cfg(feature = "ethermint")]
        let result = {
            let pubkey = deep_space::PublicKey::from_bytes(self.pubkey.to_bytes().into(), "")
                .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;
            pubkey
                .to_ethermint_address_with_prefix(prefix)
                .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?
        };
        #[cfg(not(feature = "ethermint"))]
        let result = deep_space::PublicKey::from_bytes(self.pubkey.to_bytes().into(), prefix)
            .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?
            .to_address();

        Ok(result)
    }

    async fn sign_std_msg(
        &self,
        messages: &[deep_space::Msg],
        args: deep_space::MessageArgs,
        memo: String,
    ) -> Result<Vec<u8>, GravityError> {
        let parts = self.build_tx(messages, args, memo).await?;

        let tx_raw = TxRaw {
            body_bytes: parts.body_buf,
            auth_info_bytes: parts.auth_buf,
            signatures: parts.signatures,
        };

        let mut txraw_buf = Vec::new();
        tx_raw
            .encode(&mut txraw_buf)
            .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;
        let digest = Sha256::digest(&txraw_buf);
        trace!("TXID {}", bytes_to_hex_str(&digest));

        Ok(txraw_buf)
    }

    async fn build_tx(
        &self,
        messages: &[deep_space::Msg],
        args: deep_space::MessageArgs,
        memo: String,
    ) -> Result<TxParts, GravityError> {
        let our_pubkey = self.pubkey.to_bytes();
        // Create TxBody
        let body = TxBody {
            messages: messages.iter().map(|msg| msg.clone().into()).collect(),
            memo,
            timeout_height: args.timeout_height,
            extension_options: Default::default(),
            non_critical_extension_options: Default::default(),
        };

        // A protobuf serialization of a TxBody
        let mut body_buf = Vec::new();
        body.encode(&mut body_buf)
            .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;

        let key = ProtoSecp256k1Pubkey {
            key: our_pubkey.to_vec(),
        };

        #[cfg(feature = "ethermint")]
        let pk_url = "/ethermint.crypto.v1.ethsecp256k1.PubKey";
        #[cfg(not(feature = "ethermint"))]
        let pk_url = COSMOS_PUBKEY_URL;

        let pk_any = encode_any(key, pk_url.to_string());

        let single = mode_info::Single { mode: 1 };

        let mode = Some(ModeInfo {
            sum: Some(mode_info::Sum::Single(single)),
        });

        let signer_info = SignerInfo {
            public_key: Some(pk_any),
            mode_info: mode,
            sequence: args.sequence,
        };

        let auth_info = AuthInfo {
            signer_infos: vec![signer_info],
            fee: Some(args.fee.into()),
        };

        // Protobuf serialization of `AuthInfo`
        let mut auth_buf = Vec::new();
        auth_info
            .encode(&mut auth_buf)
            .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;

        let sign_doc = SignDoc {
            body_bytes: body_buf.clone(),
            auth_info_bytes: auth_buf.clone(),
            chain_id: args.chain_id.to_string(),
            account_number: args.account_number,
        };

        // Protobuf serialization of `SignDoc`
        let mut signdoc_buf = Vec::new();
        sign_doc
            .encode(&mut signdoc_buf)
            .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;
        #[cfg(feature = "ethermint")]
        let sign_type = SignType::Ethermint;
        #[cfg(not(feature = "ethermint"))]
        let sign_type = SignType::Cosmos;
        let compact = match sign_type {
            SignType::Cosmos => {
                let digest = Sha256::digest(&signdoc_buf);
                let signed = self
                    .sign_digest(digest.into())
                    .await
                    .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;
                signed.to_vec()
            }
            SignType::Ethermint => {
                let digest = keccak256(&signdoc_buf);
                let sig = self
                    .sign_digest(digest)
                    .await
                    .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;
                let sig = rsig_from_digest_bytes_trial_recovery(&sig, digest, &self.pubkey)
                    .map_err(|e| GravityError::CosmosSignerError(Box::new(e)))?;
                let sig = rsig_to_ethsig(&sig);
                sig.to_vec()
            }
        };

        Ok(TxParts {
            body,
            body_buf,
            auth_info,
            auth_buf,
            signatures: vec![compact],
        })
    }
}
