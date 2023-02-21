use deep_space::private_key::TxParts;
use ethers::{prelude::k256::ecdsa::VerifyingKey, signers::LocalWallet};
use gravity_utils::error::GravityError;
use std::str::FromStr;
use tonic::async_trait;

#[cfg(not(feature = "ethermint"))]
use deep_space::public_key::COSMOS_PUBKEY_URL;
use deep_space::{
    error::{Bip39Error, HdWalletError, PrivateKeyError},
    private_key::{PrivateKey as InnerPrivateKey, SignType},
    Address, MessageArgs, Msg,
};

#[cfg(feature = "ethermint")]
pub const DEFAULT_HD_PATH: &str = "m/44'/60'/0'/0/0";
#[cfg(not(feature = "ethermint"))]
pub const DEFAULT_HD_PATH: &str = "m/44'/118'/0'/0/0";

/// A trait to return a public key
pub trait EthPubkey {
    fn to_public_key(&self) -> VerifyingKey;
}

impl EthPubkey for LocalWallet {
    fn to_public_key(&self) -> VerifyingKey {
        self.signer().verifying_key()
    }
}

/// A trait that captures different possible signer implementations.
#[async_trait]
pub trait CosmosSigner: Clone {
    fn to_address(&self, prefix: &str) -> Result<Address, GravityError>;
    async fn sign_std_msg(
        &self,
        messages: &[Msg],
        args: MessageArgs,
        memo: String,
    ) -> Result<Vec<u8>, GravityError>;
    async fn build_tx(
        &self,
        messages: &[Msg],
        args: MessageArgs,
        memo: String,
    ) -> Result<TxParts, GravityError>;
}

#[async_trait]
impl CosmosSigner for PrivateKey {
    fn to_address(&self, prefix: &str) -> Result<Address, GravityError> {
        self.to_address(prefix)
            .map_err(GravityError::CosmosPrivateKeyError)
    }
    async fn sign_std_msg(
        &self,
        messages: &[Msg],
        args: MessageArgs,
        memo: String,
    ) -> Result<Vec<u8>, GravityError> {
        self.do_sign_std_msg(messages, args, memo)
            .map_err(GravityError::CosmosPrivateKeyError)
    }
    async fn build_tx(
        &self,
        messages: &[Msg],
        args: MessageArgs,
        memo: String,
    ) -> Result<TxParts, GravityError> {
        self.do_build_tx(messages, args, memo)
            .map_err(GravityError::CosmosPrivateKeyError)
    }
}

/// PrivateKey wraps cosmos private key, switch between cosmos and ethermint behavior according to cargo features.
#[derive(Debug, Copy, Clone)]
pub struct PrivateKey(InnerPrivateKey);

impl FromStr for PrivateKey {
    type Err = PrivateKeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InnerPrivateKey::from_str(s).map(Self)
    }
}

impl From<PrivateKey> for InnerPrivateKey {
    fn from(val: PrivateKey) -> Self {
        val.0
    }
}

impl PrivateKey {
    pub fn from_hd_wallet_path(
        hd_path: &str,
        phrase: &str,
        passphrase: &str,
    ) -> Result<Self, PrivateKeyError> {
        InnerPrivateKey::from_hd_wallet_path(hd_path, phrase, passphrase).map(Self)
    }

    pub fn from_phrase(phrase: &str, passphrase: &str) -> Result<Self, PrivateKeyError> {
        if phrase.is_empty() {
            return Err(HdWalletError::Bip39Error(Bip39Error::BadWordCount(0)).into());
        }
        Self::from_hd_wallet_path(DEFAULT_HD_PATH, phrase, passphrase)
    }

    pub fn from_secret(secret: &[u8]) -> Self {
        Self(InnerPrivateKey::from_secret(secret))
    }

    pub fn to_address(&self, prefix: &str) -> Result<Address, PrivateKeyError> {
        #[cfg(feature = "ethermint")]
        let result = {
            let pubkey = self.0.to_public_key("")?;
            Ok(pubkey.to_ethermint_address_with_prefix(prefix)?)
        };
        #[cfg(not(feature = "ethermint"))]
        let result = self.0.to_address(prefix);

        result
    }

    pub fn do_sign_std_msg(
        &self,
        messages: &[Msg],
        args: MessageArgs,
        memo: impl Into<String>,
    ) -> Result<Vec<u8>, PrivateKeyError> {
        #[cfg(feature = "ethermint")]
        let result = self.0.sign_std_msg_ethermint(
            messages,
            args,
            memo,
            "/ethermint.crypto.v1.ethsecp256k1.PubKey",
        );
        #[cfg(not(feature = "ethermint"))]
        let result = self.0.sign_std_msg(messages, args, memo);

        result
    }

    pub fn do_build_tx(
        &self,
        messages: &[Msg],
        args: MessageArgs,
        memo: impl Into<String>,
    ) -> Result<TxParts, PrivateKeyError> {
        #[cfg(feature = "ethermint")]
        return self.0.build_tx(
            messages,
            args,
            memo,
            "/ethermint.crypto.v1.ethsecp256k1.PubKey",
            SignType::Ethermint,
        );
        #[cfg(not(feature = "ethermint"))]
        return self
            .0
            .build_tx(messages, args, memo, COSMOS_PUBKEY_URL, SignType::Cosmos);
    }
}
