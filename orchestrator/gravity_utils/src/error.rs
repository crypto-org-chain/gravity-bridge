//! for things that don't belong in the cosmos or ethereum libraries but also don't belong
//! in a function specific library
use clarity::Error as ClarityError;
use deep_space::error::AddressError as CosmosAddressError;
use deep_space::error::CosmosGrpcError;
use deep_space::error::PrivateKeyError as CosmosPrivateKeyError;
use ethers::abi::ethereum_types::FromDecStrErr as EthersParseUintError;
use ethers::abi::Error as EthersAbiError;
use ethers::contract::AbiError as EthersContractAbiError;
use ethers::prelude::errors::EtherscanError;
use ethers::prelude::gas_oracle::GasOracleError as EthersGasOracleError;
use ethers::prelude::signer::SignerMiddlewareError;
use ethers::prelude::ContractError;
use ethers::prelude::ProviderError as EthersProviderError;
use ethers::prelude::*;
use ethers::types::SignatureError as EthersSignatureError;
use num_bigint::ParseBigIntError;
use rustc_hex::FromHexError as EthersParseAddressError;
use std::fmt::{self, Debug};
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use tokio::time::error::Elapsed;
use tonic::Status;

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum GravityError<S: Signer> {
    CosmosGrpcError(CosmosGrpcError),
    CosmosAddressError(CosmosAddressError),
    CosmosPrivateKeyError(CosmosPrivateKeyError),
    EthereumBadDataError(String),
    EthereumRestError(SignerMiddlewareError<Provider<Http>, S>),
    EthersAbiError(EthersAbiError),
    EthersContractAbiError(EthersContractAbiError),
    EthersContractError(ContractError<SignerMiddleware<Provider<Http>, S>>),
    EthersGasOracleError(EthersGasOracleError),
    EthersParseAddressError(EthersParseAddressError),
    EthersParseUintError(EthersParseUintError),
    EthersProviderError(EthersProviderError),
    EthersSignatureError(EthersSignatureError),
    EthersWalletError(S::Error),
    EtherscanError(EtherscanError),
    GravityContractError(String),
    InvalidArgumentError(String),
    InvalidBridgeStateError(String),
    FailedToUpdateValset,
    EthereumContractError(String),
    InvalidOptionsError(String),
    ClarityError(ClarityError),
    TimeoutError,
    InvalidEventLogError(String),
    GravityGrpcError(Status),
    InsufficientVotingPowerToPass(String),
    ParseBigIntError(ParseBigIntError),
    ParseIntError(ParseIntError),
    FromUtf8Error(FromUtf8Error),
    OverflowError(String),
}

impl<S: Signer> fmt::Display for GravityError<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GravityError::GravityGrpcError(val) => write!(f, "Gravity gRPC error {}", val),
            GravityError::CosmosGrpcError(val) => write!(f, "Cosmos gRPC error {}", val),
            GravityError::CosmosAddressError(val) => write!(f, "Cosmos Address error {}", val),
            GravityError::CosmosPrivateKeyError(val) => {
                write!(f, "Cosmos private key error:  {}", val)
            }
            GravityError::EthereumBadDataError(val) => {
                write!(f, "Received unexpected data from Ethereum: {}", val)
            }
            GravityError::EthereumRestError(val) => write!(f, "Ethereum REST error: {}", val),
            GravityError::EthersAbiError(val) => write!(f, "Ethers ABI error: {}", val),
            GravityError::EthersContractAbiError(val) => {
                write!(f, "Ethers contract ABI error: {}", val)
            }
            GravityError::EthersContractError(val) => write!(f, "Ethers contract error: {}", val),
            GravityError::EthersGasOracleError(val) => {
                write!(f, "Ethers gas oracle error: {}", val)
            }
            GravityError::EthersParseAddressError(val) => {
                write!(f, "Ethers H160 address parse error: {}", val)
            }
            GravityError::EthersParseUintError(val) => {
                write!(f, "Ethers U256 parse error: {}", val)
            }
            GravityError::EthersProviderError(val) => write!(f, "Ethers provider error: {}", val),
            GravityError::EthersSignatureError(val) => write!(f, "Ethers signature error: {}", val),
            GravityError::EthersWalletError(val) => write!(f, "Ethers wallet error: {}", val),
            GravityError::EtherscanError(val) => write!(f, "Etherscan error: {}", val),
            GravityError::GravityContractError(val) => write!(f, "Gravity contract error: {}", val),
            GravityError::InvalidArgumentError(val) => write!(f, "Invalid argument error: {}", val),
            GravityError::InvalidOptionsError(val) => {
                write!(f, "Invalid TX options for this call {}", val)
            }
            GravityError::InvalidBridgeStateError(val) => {
                write!(f, "Invalid bridge state! {}", val)
            }
            GravityError::FailedToUpdateValset => write!(f, "ValidatorSetUpdate Failed!"),
            GravityError::TimeoutError => write!(f, "Operation timed out!"),
            GravityError::ClarityError(val) => write!(f, "Clarity Error {}", val),
            GravityError::InvalidEventLogError(val) => write!(f, "InvalidEvent: {}", val),
            GravityError::EthereumContractError(val) => {
                write!(f, "Contract operation failed: {}", val)
            }
            GravityError::InsufficientVotingPowerToPass(val) => {
                write!(f, "{}", val)
            }
            GravityError::ParseBigIntError(val) => write!(f, "Failed to parse big integer {}", val),
            GravityError::ParseIntError(val) => write!(f, "Failed to parse integer: {}", val),
            GravityError::FromUtf8Error(val) => {
                write!(f, "Failed to parse bytes to UTF-8: {}", val)
            }
            GravityError::OverflowError(val) => write!(f, "Overflow error: {}", val),
        }
    }
}

impl<S: Signer> std::error::Error for GravityError<S> {}

impl<S: Signer> From<CosmosGrpcError> for GravityError<S> {
    fn from(error: CosmosGrpcError) -> Self {
        GravityError::CosmosGrpcError(error)
    }
}

impl<S: Signer> From<CosmosAddressError> for GravityError<S> {
    fn from(error: CosmosAddressError) -> Self {
        GravityError::CosmosAddressError(error)
    }
}

impl<S: Signer> From<CosmosPrivateKeyError> for GravityError<S> {
    fn from(error: CosmosPrivateKeyError) -> Self {
        GravityError::CosmosPrivateKeyError(error)
    }
}

impl<S: Signer> From<Elapsed> for GravityError<S> {
    fn from(_error: Elapsed) -> Self {
        GravityError::TimeoutError
    }
}

impl<S: Signer> From<ClarityError> for GravityError<S> {
    fn from(error: ClarityError) -> Self {
        GravityError::ClarityError(error)
    }
}

impl<S: Signer> From<SignerMiddlewareError<Provider<Http>, S>> for GravityError<S> {
    fn from(error: SignerMiddlewareError<Provider<Http>, S>) -> Self {
        GravityError::EthereumRestError(error)
    }
}

impl<S: Signer> From<EthersAbiError> for GravityError<S> {
    fn from(error: EthersAbiError) -> Self {
        GravityError::EthersAbiError(error)
    }
}

impl<S: Signer> From<EthersContractAbiError> for GravityError<S> {
    fn from(error: EthersContractAbiError) -> Self {
        GravityError::EthersContractAbiError(error)
    }
}

impl<S: Signer> From<ContractError<SignerMiddleware<Provider<Http>, S>>> for GravityError<S> {
    fn from(error: ContractError<SignerMiddleware<Provider<Http>, S>>) -> Self {
        GravityError::EthersContractError(error)
    }
}

impl<S: Signer> From<EthersGasOracleError> for GravityError<S> {
    fn from(error: EthersGasOracleError) -> Self {
        GravityError::EthersGasOracleError(error)
    }
}

impl<S: Signer> From<EthersParseAddressError> for GravityError<S> {
    fn from(error: EthersParseAddressError) -> Self {
        GravityError::EthersParseAddressError(error)
    }
}

impl<S: Signer> From<EthersParseUintError> for GravityError<S> {
    fn from(error: EthersParseUintError) -> Self {
        GravityError::EthersParseUintError(error)
    }
}

impl<S: Signer> From<EthersProviderError> for GravityError<S> {
    fn from(error: EthersProviderError) -> Self {
        GravityError::EthersProviderError(error)
    }
}

impl<S: Signer> From<EthersSignatureError> for GravityError<S> {
    fn from(error: EthersSignatureError) -> Self {
        GravityError::EthersSignatureError(error)
    }
}

impl<S: Signer> From<EtherscanError> for GravityError<S> {
    fn from(error: EtherscanError) -> Self {
        GravityError::EtherscanError(error)
    }
}

impl<S: Signer> From<Status> for GravityError<S> {
    fn from(error: Status) -> Self {
        GravityError::GravityGrpcError(error)
    }
}

impl<S: Signer> From<ParseBigIntError> for GravityError<S> {
    fn from(error: ParseBigIntError) -> Self {
        GravityError::ParseBigIntError(error)
    }
}

impl<S: Signer> From<ParseIntError> for GravityError<S> {
    fn from(error: ParseIntError) -> Self {
        GravityError::ParseIntError(error)
    }
}

impl<S: Signer> From<FromUtf8Error> for GravityError<S> {
    fn from(error: FromUtf8Error) -> Self {
        GravityError::FromUtf8Error(error)
    }
}
