pub use gravity::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod gravity {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Gravity was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_gravityId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_powerThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayerAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BatchTimedOut\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectCheckpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cumulativePower\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"powerThreshold\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InsufficientPower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"currentNonce\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidBatchNonce\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidLogicCallFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"currentNonce\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidLogicCallNonce\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidLogicCallTransfers\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSendToCosmos\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"currentNonce\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidValsetNonce\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LogicCallTimedOut\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MalformedBatch\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MalformedCurrentValidatorSet\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MalformedNewValidatorSet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"anyoneCanRelay\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AnyoneCanRelay\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_cosmosDenom\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"_tokenContract\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_eventNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ERC20DeployedEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_invalidationId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_invalidationNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"_returnData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_eventNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogicCallEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"migration\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Migration\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenContract\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"_destination\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_eventNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SendToCosmosEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_batchNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_eventNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransactionBatchExecutedEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_newValsetNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_eventNonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_rewardAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"_rewardToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"_validators\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"_powers\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ValsetUpdatedEvent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MIGRATION_PERIOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RELAYER\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RELAYER_ADMIN\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"anyoneCanRelay\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_cosmosDenom\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployERC20\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_erc20Address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBatchNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_invalidation_id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastLogicCallNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_newGravityAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isCosmosToken\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrateToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"migration\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"migrationHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_newDestination\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemVoucher\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferSelf\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_destination\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendToCosmos\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_destination\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendToCronos\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_anyoneCanRelay\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAnyoneCanRelay\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startMigration\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_RevertedVouchers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"tokenContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_gravityId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_invalidationMapping\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_lastBatchNonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_lastEventNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_lastRevertedNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_lastValsetCheckpoint\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_lastValsetNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state_powerThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stopMigration\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ValsetArgs\",\"name\":\"_currentValset\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"valsetNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct ValSignature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}]},{\"internalType\":\"struct PaymentArgs\",\"name\":\"_payments\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"destinations\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"fees\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"feePaymentAddress\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"_batchNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_tokenContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_batchTimeout\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBatch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ValsetArgs\",\"name\":\"_currentValset\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"valsetNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct ValSignature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"_paymentAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct LogicCallArgs\",\"name\":\"_args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[]\",\"name\":\"transferAmounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"transferTokenContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"feeAmounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"feeTokenContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"logicContractAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timeOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"invalidationId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"invalidationNonce\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitLogicCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ValsetArgs\",\"name\":\"_currentValset\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"valsetNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct ValSignature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"_theHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_powerThreshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"testCheckValidatorSignatures\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ValsetArgs\",\"name\":\"_valsetArgs\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"valsetNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"_gravityId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"testMakeCheckpoint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferRelayerAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ValsetArgs\",\"name\":\"_newValset\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"valsetNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct ValsetArgs\",\"name\":\"_currentValset\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"validators\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"powers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"valsetNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct ValSignature[]\",\"name\":\"_sigs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateValset\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static GRAVITY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Gravity<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Gravity<M> {
        fn clone(&self) -> Self {
            Gravity(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Gravity<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Gravity<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Gravity))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Gravity<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GRAVITY_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
        pub fn default_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MIGRATION_PERIOD` (0x28b7c7f0) function"]
        pub fn migration_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([40, 183, 199, 240], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RELAYER` (0x2483e715) function"]
        pub fn relayer(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 131, 231, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RELAYER_ADMIN` (0xa97ccfd3) function"]
        pub fn relayer_admin(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([169, 124, 207, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `anyoneCanRelay` (0x3c032712) function"]
        pub fn anyone_can_relay(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([60, 3, 39, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployERC20` (0xf7955637) function"]
        pub fn deploy_erc20(
            &self,
            cosmos_denom: String,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 149, 86, 55], (cosmos_denom, name, symbol, decimals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastBatchNonce` (0x011b2174) function"]
        pub fn last_batch_nonce(
            &self,
            erc_20_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([1, 27, 33, 116], erc_20_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastLogicCallNonce` (0xc9d194d5) function"]
        pub fn last_logic_call_nonce(
            &self,
            invalidation_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([201, 209, 148, 213], invalidation_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrateToken` (0xd3e39557) function"]
        pub fn migrate_token(
            &self,
            token_contract: ethers::core::types::Address,
            new_gravity_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            is_cosmos_token: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [211, 227, 149, 87],
                    (token_contract, new_gravity_address, amount, is_cosmos_token),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migration` (0x1705a3bd) function"]
        pub fn migration(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 5, 163, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrationHeight` (0x60969e86) function"]
        pub fn migration_height(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([96, 150, 158, 134], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemVoucher` (0x33cbf59d) function"]
        pub fn redeem_voucher(
            &self,
            nonce: ethers::core::types::U256,
            new_destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 203, 245, 157], (nonce, new_destination))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferSelf` (0x2140db05) function"]
        pub fn safe_transfer_self(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 64, 219, 5], (token, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendToCosmos` (0x1ffbe7f9) function"]
        pub fn send_to_cosmos(
            &self,
            token_contract: ethers::core::types::Address,
            destination: [u8; 32],
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 251, 231, 249], (token_contract, destination, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendToCronos` (0x53c5060c) function"]
        pub fn send_to_cronos(
            &self,
            token_contract: ethers::core::types::Address,
            destination: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 197, 6, 12], (token_contract, destination, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAnyoneCanRelay` (0x2452fb40) function"]
        pub fn set_anyone_can_relay(
            &self,
            anyone_can_relay: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 82, 251, 64], anyone_can_relay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startMigration` (0x18264f33) function"]
        pub fn start_migration(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 38, 79, 51], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_RevertedVouchers` (0x7d55180b) function"]
        pub fn state_reverted_vouchers(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([125, 85, 24, 11], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_gravityId` (0xbdda81d4) function"]
        pub fn state_gravity_id(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([189, 218, 129, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_invalidationMapping` (0x7dfb6f86) function"]
        pub fn state_invalidation_mapping(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([125, 251, 111, 134], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_lastBatchNonces` (0xdf97174b) function"]
        pub fn state_last_batch_nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 151, 23, 75], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_lastEventNonce` (0x73b20547) function"]
        pub fn state_last_event_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([115, 178, 5, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_lastRevertedNonce` (0xf16d67cc) function"]
        pub fn state_last_reverted_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([241, 109, 103, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_lastValsetCheckpoint` (0xf2b53307) function"]
        pub fn state_last_valset_checkpoint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([242, 181, 51, 7], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_lastValsetNonce` (0xb56561fe) function"]
        pub fn state_last_valset_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([181, 101, 97, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `state_powerThreshold` (0xe5a2b5d2) function"]
        pub fn state_power_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([229, 162, 181, 210], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stopMigration` (0xe19a7bc8) function"]
        pub fn stop_migration(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 123, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitBatch` (0xf0928774) function"]
        pub fn submit_batch(
            &self,
            current_valset: ValsetArgs,
            sigs: ::std::vec::Vec<ValSignature>,
            payments: PaymentArgs,
            batch_nonce: ethers::core::types::U256,
            token_contract: ethers::core::types::Address,
            batch_timeout: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [240, 146, 135, 116],
                    (
                        current_valset,
                        sigs,
                        payments,
                        batch_nonce,
                        token_contract,
                        batch_timeout,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitLogicCall` (0x38040a33) function"]
        pub fn submit_logic_call(
            &self,
            current_valset: ValsetArgs,
            sigs: ::std::vec::Vec<ValSignature>,
            payment_address: ethers::core::types::Address,
            args: LogicCallArgs,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [56, 4, 10, 51],
                    (current_valset, sigs, payment_address, args),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testCheckValidatorSignatures` (0x00901153) function"]
        pub fn test_check_validator_signatures(
            &self,
            current_valset: ValsetArgs,
            sigs: ::std::vec::Vec<ValSignature>,
            the_hash: [u8; 32],
            power_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 144, 17, 83],
                    (current_valset, sigs, the_hash, power_threshold),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testMakeCheckpoint` (0x01031525) function"]
        pub fn test_make_checkpoint(
            &self,
            valset_args: ValsetArgs,
            gravity_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 3, 21, 37], (valset_args, gravity_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferRelayerAdmin` (0x82e2cfbe) function"]
        pub fn transfer_relayer_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 226, 207, 190], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateValset` (0xaca6b1c1) function"]
        pub fn update_valset(
            &self,
            new_valset: ValsetArgs,
            current_valset: ValsetArgs,
            sigs: ::std::vec::Vec<ValSignature>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 166, 177, 193], (new_valset, current_valset, sigs))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AnyoneCanRelay` event"]
        pub fn anyone_can_relay_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AnyoneCanRelayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ERC20DeployedEvent` event"]
        pub fn erc20_deployed_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, Erc20DeployedEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogicCallEvent` event"]
        pub fn logic_call_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogicCallEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Migration` event"]
        pub fn migration_filter(&self) -> ethers::contract::builders::Event<M, MigrationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SendToCosmosEvent` event"]
        pub fn send_to_cosmos_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SendToCosmosEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransactionBatchExecutedEvent` event"]
        pub fn transaction_batch_executed_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransactionBatchExecutedEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ValsetUpdatedEvent` event"]
        pub fn valset_updated_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ValsetUpdatedEventFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, GravityEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Gravity<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `BatchTimedOut` with signature `BatchTimedOut()` and selector `[17, 114, 76, 198]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "BatchTimedOut", abi = "BatchTimedOut()")]
    pub struct BatchTimedOut;
    #[doc = "Custom Error type `IncorrectCheckpoint` with signature `IncorrectCheckpoint()` and selector `[114, 58, 52, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "IncorrectCheckpoint", abi = "IncorrectCheckpoint()")]
    pub struct IncorrectCheckpoint;
    #[doc = "Custom Error type `InsufficientPower` with signature `InsufficientPower(uint256,uint256)` and selector `[0, 191, 182, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "InsufficientPower", abi = "InsufficientPower(uint256,uint256)")]
    pub struct InsufficientPower {
        pub cumulative_power: ethers::core::types::U256,
        pub power_threshold: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `InvalidBatchNonce` with signature `InvalidBatchNonce(uint256,uint256)` and selector `[247, 249, 32, 173]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "InvalidBatchNonce", abi = "InvalidBatchNonce(uint256,uint256)")]
    pub struct InvalidBatchNonce {
        pub new_nonce: ethers::core::types::U256,
        pub current_nonce: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `InvalidLogicCallFees` with signature `InvalidLogicCallFees()` and selector `[72, 41, 36, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "InvalidLogicCallFees", abi = "InvalidLogicCallFees()")]
    pub struct InvalidLogicCallFees;
    #[doc = "Custom Error type `InvalidLogicCallNonce` with signature `InvalidLogicCallNonce(uint256,uint256)` and selector `[1, 40, 79, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(
        name = "InvalidLogicCallNonce",
        abi = "InvalidLogicCallNonce(uint256,uint256)"
    )]
    pub struct InvalidLogicCallNonce {
        pub new_nonce: ethers::core::types::U256,
        pub current_nonce: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `InvalidLogicCallTransfers` with signature `InvalidLogicCallTransfers()` and selector `[133, 49, 82, 162]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(
        name = "InvalidLogicCallTransfers",
        abi = "InvalidLogicCallTransfers()"
    )]
    pub struct InvalidLogicCallTransfers;
    #[doc = "Custom Error type `InvalidSendToCosmos` with signature `InvalidSendToCosmos()` and selector `[33, 115, 157, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "InvalidSendToCosmos", abi = "InvalidSendToCosmos()")]
    pub struct InvalidSendToCosmos;
    #[doc = "Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `[139, 170, 87, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    #[doc = "Custom Error type `InvalidValsetNonce` with signature `InvalidValsetNonce(uint256,uint256)` and selector `[224, 232, 237, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(
        name = "InvalidValsetNonce",
        abi = "InvalidValsetNonce(uint256,uint256)"
    )]
    pub struct InvalidValsetNonce {
        pub new_nonce: ethers::core::types::U256,
        pub current_nonce: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `LogicCallTimedOut` with signature `LogicCallTimedOut()` and selector `[188, 243, 124, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "LogicCallTimedOut", abi = "LogicCallTimedOut()")]
    pub struct LogicCallTimedOut;
    #[doc = "Custom Error type `MalformedBatch` with signature `MalformedBatch()` and selector `[193, 249, 126, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "MalformedBatch", abi = "MalformedBatch()")]
    pub struct MalformedBatch;
    #[doc = "Custom Error type `MalformedCurrentValidatorSet` with signature `MalformedCurrentValidatorSet()` and selector `[198, 97, 123, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(
        name = "MalformedCurrentValidatorSet",
        abi = "MalformedCurrentValidatorSet()"
    )]
    pub struct MalformedCurrentValidatorSet;
    #[doc = "Custom Error type `MalformedNewValidatorSet` with signature `MalformedNewValidatorSet()` and selector `[192, 27, 160, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[etherror(name = "MalformedNewValidatorSet", abi = "MalformedNewValidatorSet()")]
    pub struct MalformedNewValidatorSet;
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum GravityErrors {
        BatchTimedOut(BatchTimedOut),
        IncorrectCheckpoint(IncorrectCheckpoint),
        InsufficientPower(InsufficientPower),
        InvalidBatchNonce(InvalidBatchNonce),
        InvalidLogicCallFees(InvalidLogicCallFees),
        InvalidLogicCallNonce(InvalidLogicCallNonce),
        InvalidLogicCallTransfers(InvalidLogicCallTransfers),
        InvalidSendToCosmos(InvalidSendToCosmos),
        InvalidSignature(InvalidSignature),
        InvalidValsetNonce(InvalidValsetNonce),
        LogicCallTimedOut(LogicCallTimedOut),
        MalformedBatch(MalformedBatch),
        MalformedCurrentValidatorSet(MalformedCurrentValidatorSet),
        MalformedNewValidatorSet(MalformedNewValidatorSet),
    }
    impl ethers::core::abi::AbiDecode for GravityErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BatchTimedOut as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::BatchTimedOut(decoded));
            }
            if let Ok(decoded) =
                <IncorrectCheckpoint as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::IncorrectCheckpoint(decoded));
            }
            if let Ok(decoded) =
                <InsufficientPower as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InsufficientPower(decoded));
            }
            if let Ok(decoded) =
                <InvalidBatchNonce as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidBatchNonce(decoded));
            }
            if let Ok(decoded) =
                <InvalidLogicCallFees as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidLogicCallFees(decoded));
            }
            if let Ok(decoded) =
                <InvalidLogicCallNonce as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidLogicCallNonce(decoded));
            }
            if let Ok(decoded) =
                <InvalidLogicCallTransfers as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidLogicCallTransfers(decoded));
            }
            if let Ok(decoded) =
                <InvalidSendToCosmos as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidSendToCosmos(decoded));
            }
            if let Ok(decoded) =
                <InvalidSignature as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidSignature(decoded));
            }
            if let Ok(decoded) =
                <InvalidValsetNonce as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::InvalidValsetNonce(decoded));
            }
            if let Ok(decoded) =
                <LogicCallTimedOut as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::LogicCallTimedOut(decoded));
            }
            if let Ok(decoded) =
                <MalformedBatch as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::MalformedBatch(decoded));
            }
            if let Ok(decoded) =
                <MalformedCurrentValidatorSet as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GravityErrors::MalformedCurrentValidatorSet(decoded));
            }
            if let Ok(decoded) =
                <MalformedNewValidatorSet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityErrors::MalformedNewValidatorSet(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GravityErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                GravityErrors::BatchTimedOut(element) => element.encode(),
                GravityErrors::IncorrectCheckpoint(element) => element.encode(),
                GravityErrors::InsufficientPower(element) => element.encode(),
                GravityErrors::InvalidBatchNonce(element) => element.encode(),
                GravityErrors::InvalidLogicCallFees(element) => element.encode(),
                GravityErrors::InvalidLogicCallNonce(element) => element.encode(),
                GravityErrors::InvalidLogicCallTransfers(element) => element.encode(),
                GravityErrors::InvalidSendToCosmos(element) => element.encode(),
                GravityErrors::InvalidSignature(element) => element.encode(),
                GravityErrors::InvalidValsetNonce(element) => element.encode(),
                GravityErrors::LogicCallTimedOut(element) => element.encode(),
                GravityErrors::MalformedBatch(element) => element.encode(),
                GravityErrors::MalformedCurrentValidatorSet(element) => element.encode(),
                GravityErrors::MalformedNewValidatorSet(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GravityErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GravityErrors::BatchTimedOut(element) => element.fmt(f),
                GravityErrors::IncorrectCheckpoint(element) => element.fmt(f),
                GravityErrors::InsufficientPower(element) => element.fmt(f),
                GravityErrors::InvalidBatchNonce(element) => element.fmt(f),
                GravityErrors::InvalidLogicCallFees(element) => element.fmt(f),
                GravityErrors::InvalidLogicCallNonce(element) => element.fmt(f),
                GravityErrors::InvalidLogicCallTransfers(element) => element.fmt(f),
                GravityErrors::InvalidSendToCosmos(element) => element.fmt(f),
                GravityErrors::InvalidSignature(element) => element.fmt(f),
                GravityErrors::InvalidValsetNonce(element) => element.fmt(f),
                GravityErrors::LogicCallTimedOut(element) => element.fmt(f),
                GravityErrors::MalformedBatch(element) => element.fmt(f),
                GravityErrors::MalformedCurrentValidatorSet(element) => element.fmt(f),
                GravityErrors::MalformedNewValidatorSet(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BatchTimedOut> for GravityErrors {
        fn from(var: BatchTimedOut) -> Self {
            GravityErrors::BatchTimedOut(var)
        }
    }
    impl ::std::convert::From<IncorrectCheckpoint> for GravityErrors {
        fn from(var: IncorrectCheckpoint) -> Self {
            GravityErrors::IncorrectCheckpoint(var)
        }
    }
    impl ::std::convert::From<InsufficientPower> for GravityErrors {
        fn from(var: InsufficientPower) -> Self {
            GravityErrors::InsufficientPower(var)
        }
    }
    impl ::std::convert::From<InvalidBatchNonce> for GravityErrors {
        fn from(var: InvalidBatchNonce) -> Self {
            GravityErrors::InvalidBatchNonce(var)
        }
    }
    impl ::std::convert::From<InvalidLogicCallFees> for GravityErrors {
        fn from(var: InvalidLogicCallFees) -> Self {
            GravityErrors::InvalidLogicCallFees(var)
        }
    }
    impl ::std::convert::From<InvalidLogicCallNonce> for GravityErrors {
        fn from(var: InvalidLogicCallNonce) -> Self {
            GravityErrors::InvalidLogicCallNonce(var)
        }
    }
    impl ::std::convert::From<InvalidLogicCallTransfers> for GravityErrors {
        fn from(var: InvalidLogicCallTransfers) -> Self {
            GravityErrors::InvalidLogicCallTransfers(var)
        }
    }
    impl ::std::convert::From<InvalidSendToCosmos> for GravityErrors {
        fn from(var: InvalidSendToCosmos) -> Self {
            GravityErrors::InvalidSendToCosmos(var)
        }
    }
    impl ::std::convert::From<InvalidSignature> for GravityErrors {
        fn from(var: InvalidSignature) -> Self {
            GravityErrors::InvalidSignature(var)
        }
    }
    impl ::std::convert::From<InvalidValsetNonce> for GravityErrors {
        fn from(var: InvalidValsetNonce) -> Self {
            GravityErrors::InvalidValsetNonce(var)
        }
    }
    impl ::std::convert::From<LogicCallTimedOut> for GravityErrors {
        fn from(var: LogicCallTimedOut) -> Self {
            GravityErrors::LogicCallTimedOut(var)
        }
    }
    impl ::std::convert::From<MalformedBatch> for GravityErrors {
        fn from(var: MalformedBatch) -> Self {
            GravityErrors::MalformedBatch(var)
        }
    }
    impl ::std::convert::From<MalformedCurrentValidatorSet> for GravityErrors {
        fn from(var: MalformedCurrentValidatorSet) -> Self {
            GravityErrors::MalformedCurrentValidatorSet(var)
        }
    }
    impl ::std::convert::From<MalformedNewValidatorSet> for GravityErrors {
        fn from(var: MalformedNewValidatorSet) -> Self {
            GravityErrors::MalformedNewValidatorSet(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(name = "AnyoneCanRelay", abi = "AnyoneCanRelay(bool)")]
    pub struct AnyoneCanRelayFilter {
        pub anyone_can_relay: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "ERC20DeployedEvent",
        abi = "ERC20DeployedEvent(string,address,string,string,uint8,uint256)"
    )]
    pub struct Erc20DeployedEventFilter {
        pub cosmos_denom: String,
        #[ethevent(indexed)]
        pub token_contract: ethers::core::types::Address,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
        pub event_nonce: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "LogicCallEvent",
        abi = "LogicCallEvent(bytes32,uint256,bytes,uint256)"
    )]
    pub struct LogicCallEventFilter {
        pub invalidation_id: [u8; 32],
        pub invalidation_nonce: ethers::core::types::U256,
        pub return_data: ethers::core::types::Bytes,
        pub event_nonce: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(name = "Migration", abi = "Migration(bool)")]
    pub struct MigrationFilter {
        pub migration: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "SendToCosmosEvent",
        abi = "SendToCosmosEvent(address,address,bytes32,uint256,uint256)"
    )]
    pub struct SendToCosmosEventFilter {
        #[ethevent(indexed)]
        pub token_contract: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: [u8; 32],
        pub amount: ethers::core::types::U256,
        pub event_nonce: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "TransactionBatchExecutedEvent",
        abi = "TransactionBatchExecutedEvent(uint256,address,uint256)"
    )]
    pub struct TransactionBatchExecutedEventFilter {
        #[ethevent(indexed)]
        pub batch_nonce: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub event_nonce: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethevent(
        name = "ValsetUpdatedEvent",
        abi = "ValsetUpdatedEvent(uint256,uint256,uint256,address,address[],uint256[])"
    )]
    pub struct ValsetUpdatedEventFilter {
        #[ethevent(indexed)]
        pub new_valset_nonce: ethers::core::types::U256,
        pub event_nonce: ethers::core::types::U256,
        pub reward_amount: ethers::core::types::U256,
        pub reward_token: ethers::core::types::Address,
        pub validators: Vec<ethers::core::types::Address>,
        pub powers: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum GravityEvents {
        AnyoneCanRelayFilter(AnyoneCanRelayFilter),
        Erc20DeployedEventFilter(Erc20DeployedEventFilter),
        LogicCallEventFilter(LogicCallEventFilter),
        MigrationFilter(MigrationFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SendToCosmosEventFilter(SendToCosmosEventFilter),
        TransactionBatchExecutedEventFilter(TransactionBatchExecutedEventFilter),
        UnpausedFilter(UnpausedFilter),
        ValsetUpdatedEventFilter(ValsetUpdatedEventFilter),
    }
    impl ethers::contract::EthLogDecode for GravityEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnyoneCanRelayFilter::decode_log(log) {
                return Ok(GravityEvents::AnyoneCanRelayFilter(decoded));
            }
            if let Ok(decoded) = Erc20DeployedEventFilter::decode_log(log) {
                return Ok(GravityEvents::Erc20DeployedEventFilter(decoded));
            }
            if let Ok(decoded) = LogicCallEventFilter::decode_log(log) {
                return Ok(GravityEvents::LogicCallEventFilter(decoded));
            }
            if let Ok(decoded) = MigrationFilter::decode_log(log) {
                return Ok(GravityEvents::MigrationFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(GravityEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GravityEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(GravityEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(GravityEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(GravityEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SendToCosmosEventFilter::decode_log(log) {
                return Ok(GravityEvents::SendToCosmosEventFilter(decoded));
            }
            if let Ok(decoded) = TransactionBatchExecutedEventFilter::decode_log(log) {
                return Ok(GravityEvents::TransactionBatchExecutedEventFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(GravityEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = ValsetUpdatedEventFilter::decode_log(log) {
                return Ok(GravityEvents::ValsetUpdatedEventFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for GravityEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GravityEvents::AnyoneCanRelayFilter(element) => element.fmt(f),
                GravityEvents::Erc20DeployedEventFilter(element) => element.fmt(f),
                GravityEvents::LogicCallEventFilter(element) => element.fmt(f),
                GravityEvents::MigrationFilter(element) => element.fmt(f),
                GravityEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                GravityEvents::PausedFilter(element) => element.fmt(f),
                GravityEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                GravityEvents::RoleGrantedFilter(element) => element.fmt(f),
                GravityEvents::RoleRevokedFilter(element) => element.fmt(f),
                GravityEvents::SendToCosmosEventFilter(element) => element.fmt(f),
                GravityEvents::TransactionBatchExecutedEventFilter(element) => element.fmt(f),
                GravityEvents::UnpausedFilter(element) => element.fmt(f),
                GravityEvents::ValsetUpdatedEventFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `MIGRATION_PERIOD` function with signature `MIGRATION_PERIOD()` and selector `[40, 183, 199, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "MIGRATION_PERIOD", abi = "MIGRATION_PERIOD()")]
    pub struct MigrationPeriodCall;
    #[doc = "Container type for all input parameters for the `RELAYER` function with signature `RELAYER()` and selector `[36, 131, 231, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "RELAYER", abi = "RELAYER()")]
    pub struct RelayerCall;
    #[doc = "Container type for all input parameters for the `RELAYER_ADMIN` function with signature `RELAYER_ADMIN()` and selector `[169, 124, 207, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "RELAYER_ADMIN", abi = "RELAYER_ADMIN()")]
    pub struct RelayerAdminCall;
    #[doc = "Container type for all input parameters for the `anyoneCanRelay` function with signature `anyoneCanRelay()` and selector `[60, 3, 39, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "anyoneCanRelay", abi = "anyoneCanRelay()")]
    pub struct AnyoneCanRelayCall;
    #[doc = "Container type for all input parameters for the `deployERC20` function with signature `deployERC20(string,string,string,uint8)` and selector `[247, 149, 86, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "deployERC20", abi = "deployERC20(string,string,string,uint8)")]
    pub struct DeployERC20Call {
        pub cosmos_denom: String,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lastBatchNonce` function with signature `lastBatchNonce(address)` and selector `[1, 27, 33, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "lastBatchNonce", abi = "lastBatchNonce(address)")]
    pub struct LastBatchNonceCall {
        pub erc_20_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lastLogicCallNonce` function with signature `lastLogicCallNonce(bytes32)` and selector `[201, 209, 148, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "lastLogicCallNonce", abi = "lastLogicCallNonce(bytes32)")]
    pub struct LastLogicCallNonceCall {
        pub invalidation_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `migrateToken` function with signature `migrateToken(address,address,uint256,bool)` and selector `[211, 227, 149, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "migrateToken",
        abi = "migrateToken(address,address,uint256,bool)"
    )]
    pub struct MigrateTokenCall {
        pub token_contract: ethers::core::types::Address,
        pub new_gravity_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub is_cosmos_token: bool,
    }
    #[doc = "Container type for all input parameters for the `migration` function with signature `migration()` and selector `[23, 5, 163, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "migration", abi = "migration()")]
    pub struct MigrationCall;
    #[doc = "Container type for all input parameters for the `migrationHeight` function with signature `migrationHeight()` and selector `[96, 150, 158, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "migrationHeight", abi = "migrationHeight()")]
    pub struct MigrationHeightCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `pause` function with signature `pause()` and selector `[132, 86, 203, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `redeemVoucher` function with signature `redeemVoucher(uint256,address)` and selector `[51, 203, 245, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "redeemVoucher", abi = "redeemVoucher(uint256,address)")]
    pub struct RedeemVoucherCall {
        pub nonce: ethers::core::types::U256,
        pub new_destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeTransferSelf` function with signature `safeTransferSelf(address,address,uint256)` and selector `[33, 64, 219, 5]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "safeTransferSelf",
        abi = "safeTransferSelf(address,address,uint256)"
    )]
    pub struct SafeTransferSelfCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sendToCosmos` function with signature `sendToCosmos(address,bytes32,uint256)` and selector `[31, 251, 231, 249]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "sendToCosmos", abi = "sendToCosmos(address,bytes32,uint256)")]
    pub struct SendToCosmosCall {
        pub token_contract: ethers::core::types::Address,
        pub destination: [u8; 32],
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sendToCronos` function with signature `sendToCronos(address,address,uint256)` and selector `[83, 197, 6, 12]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "sendToCronos", abi = "sendToCronos(address,address,uint256)")]
    pub struct SendToCronosCall {
        pub token_contract: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setAnyoneCanRelay` function with signature `setAnyoneCanRelay(bool)` and selector `[36, 82, 251, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "setAnyoneCanRelay", abi = "setAnyoneCanRelay(bool)")]
    pub struct SetAnyoneCanRelayCall {
        pub anyone_can_relay: bool,
    }
    #[doc = "Container type for all input parameters for the `startMigration` function with signature `startMigration()` and selector `[24, 38, 79, 51]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "startMigration", abi = "startMigration()")]
    pub struct StartMigrationCall;
    #[doc = "Container type for all input parameters for the `state_RevertedVouchers` function with signature `state_RevertedVouchers(uint256)` and selector `[125, 85, 24, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "state_RevertedVouchers",
        abi = "state_RevertedVouchers(uint256)"
    )]
    pub struct StateRevertedVouchersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `state_gravityId` function with signature `state_gravityId()` and selector `[189, 218, 129, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "state_gravityId", abi = "state_gravityId()")]
    pub struct StateGravityIdCall;
    #[doc = "Container type for all input parameters for the `state_invalidationMapping` function with signature `state_invalidationMapping(bytes32)` and selector `[125, 251, 111, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "state_invalidationMapping",
        abi = "state_invalidationMapping(bytes32)"
    )]
    pub struct StateInvalidationMappingCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `state_lastBatchNonces` function with signature `state_lastBatchNonces(address)` and selector `[223, 151, 23, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "state_lastBatchNonces", abi = "state_lastBatchNonces(address)")]
    pub struct StateLastBatchNoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `state_lastEventNonce` function with signature `state_lastEventNonce()` and selector `[115, 178, 5, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "state_lastEventNonce", abi = "state_lastEventNonce()")]
    pub struct StateLastEventNonceCall;
    #[doc = "Container type for all input parameters for the `state_lastRevertedNonce` function with signature `state_lastRevertedNonce()` and selector `[241, 109, 103, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "state_lastRevertedNonce", abi = "state_lastRevertedNonce()")]
    pub struct StateLastRevertedNonceCall;
    #[doc = "Container type for all input parameters for the `state_lastValsetCheckpoint` function with signature `state_lastValsetCheckpoint()` and selector `[242, 181, 51, 7]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "state_lastValsetCheckpoint",
        abi = "state_lastValsetCheckpoint()"
    )]
    pub struct StateLastValsetCheckpointCall;
    #[doc = "Container type for all input parameters for the `state_lastValsetNonce` function with signature `state_lastValsetNonce()` and selector `[181, 101, 97, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "state_lastValsetNonce", abi = "state_lastValsetNonce()")]
    pub struct StateLastValsetNonceCall;
    #[doc = "Container type for all input parameters for the `state_powerThreshold` function with signature `state_powerThreshold()` and selector `[229, 162, 181, 210]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "state_powerThreshold", abi = "state_powerThreshold()")]
    pub struct StatePowerThresholdCall;
    #[doc = "Container type for all input parameters for the `stopMigration` function with signature `stopMigration()` and selector `[225, 154, 123, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "stopMigration", abi = "stopMigration()")]
    pub struct StopMigrationCall;
    #[doc = "Container type for all input parameters for the `submitBatch` function with signature `submitBatch((address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[],(uint256[],address[],uint256[],address),uint256,address,uint256)` and selector `[240, 146, 135, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "submitBatch",
        abi = "submitBatch((address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[],(uint256[],address[],uint256[],address),uint256,address,uint256)"
    )]
    pub struct SubmitBatchCall {
        pub current_valset: ValsetArgs,
        pub sigs: ::std::vec::Vec<ValSignature>,
        pub payments: PaymentArgs,
        pub batch_nonce: ethers::core::types::U256,
        pub token_contract: ethers::core::types::Address,
        pub batch_timeout: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `submitLogicCall` function with signature `submitLogicCall((address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[],address,(uint256[],address[],uint256[],address[],address,bytes,uint256,bytes32,uint256))` and selector `[56, 4, 10, 51]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "submitLogicCall",
        abi = "submitLogicCall((address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[],address,(uint256[],address[],uint256[],address[],address,bytes,uint256,bytes32,uint256))"
    )]
    pub struct SubmitLogicCallCall {
        pub current_valset: ValsetArgs,
        pub sigs: ::std::vec::Vec<ValSignature>,
        pub payment_address: ethers::core::types::Address,
        pub args: LogicCallArgs,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `testCheckValidatorSignatures` function with signature `testCheckValidatorSignatures((address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[],bytes32,uint256)` and selector `[0, 144, 17, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "testCheckValidatorSignatures",
        abi = "testCheckValidatorSignatures((address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[],bytes32,uint256)"
    )]
    pub struct TestCheckValidatorSignaturesCall {
        pub current_valset: ValsetArgs,
        pub sigs: ::std::vec::Vec<ValSignature>,
        pub the_hash: [u8; 32],
        pub power_threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `testMakeCheckpoint` function with signature `testMakeCheckpoint((address[],uint256[],uint256,uint256,address),bytes32)` and selector `[1, 3, 21, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "testMakeCheckpoint",
        abi = "testMakeCheckpoint((address[],uint256[],uint256,uint256,address),bytes32)"
    )]
    pub struct TestMakeCheckpointCall {
        pub valset_args: ValsetArgs,
        pub gravity_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferRelayerAdmin` function with signature `transferRelayerAdmin(address)` and selector `[130, 226, 207, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "transferRelayerAdmin", abi = "transferRelayerAdmin(address)")]
    pub struct TransferRelayerAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[doc = "Container type for all input parameters for the `updateValset` function with signature `updateValset((address[],uint256[],uint256,uint256,address),(address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[])` and selector `[172, 166, 177, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    #[ethcall(
        name = "updateValset",
        abi = "updateValset((address[],uint256[],uint256,uint256,address),(address[],uint256[],uint256,uint256,address),(uint8,bytes32,bytes32)[])"
    )]
    pub struct UpdateValsetCall {
        pub new_valset: ValsetArgs,
        pub current_valset: ValsetArgs,
        pub sigs: ::std::vec::Vec<ValSignature>,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub enum GravityCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MigrationPeriod(MigrationPeriodCall),
        Relayer(RelayerCall),
        RelayerAdmin(RelayerAdminCall),
        AnyoneCanRelay(AnyoneCanRelayCall),
        DeployERC20(DeployERC20Call),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        LastBatchNonce(LastBatchNonceCall),
        LastLogicCallNonce(LastLogicCallNonceCall),
        MigrateToken(MigrateTokenCall),
        Migration(MigrationCall),
        MigrationHeight(MigrationHeightCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        RedeemVoucher(RedeemVoucherCall),
        RenounceOwnership(RenounceOwnershipCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SafeTransferSelf(SafeTransferSelfCall),
        SendToCosmos(SendToCosmosCall),
        SendToCronos(SendToCronosCall),
        SetAnyoneCanRelay(SetAnyoneCanRelayCall),
        StartMigration(StartMigrationCall),
        StateRevertedVouchers(StateRevertedVouchersCall),
        StateGravityId(StateGravityIdCall),
        StateInvalidationMapping(StateInvalidationMappingCall),
        StateLastBatchNonces(StateLastBatchNoncesCall),
        StateLastEventNonce(StateLastEventNonceCall),
        StateLastRevertedNonce(StateLastRevertedNonceCall),
        StateLastValsetCheckpoint(StateLastValsetCheckpointCall),
        StateLastValsetNonce(StateLastValsetNonceCall),
        StatePowerThreshold(StatePowerThresholdCall),
        StopMigration(StopMigrationCall),
        SubmitBatch(SubmitBatchCall),
        SubmitLogicCall(SubmitLogicCallCall),
        SupportsInterface(SupportsInterfaceCall),
        TestCheckValidatorSignatures(TestCheckValidatorSignaturesCall),
        TestMakeCheckpoint(TestMakeCheckpointCall),
        TransferOwnership(TransferOwnershipCall),
        TransferRelayerAdmin(TransferRelayerAdminCall),
        Unpause(UnpauseCall),
        UpdateValset(UpdateValsetCall),
    }
    impl ethers::core::abi::AbiDecode for GravityCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <MigrationPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::MigrationPeriod(decoded));
            }
            if let Ok(decoded) =
                <RelayerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::Relayer(decoded));
            }
            if let Ok(decoded) =
                <RelayerAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::RelayerAdmin(decoded));
            }
            if let Ok(decoded) =
                <AnyoneCanRelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::AnyoneCanRelay(decoded));
            }
            if let Ok(decoded) =
                <DeployERC20Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::DeployERC20(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <LastBatchNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::LastBatchNonce(decoded));
            }
            if let Ok(decoded) =
                <LastLogicCallNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::LastLogicCallNonce(decoded));
            }
            if let Ok(decoded) =
                <MigrateTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::MigrateToken(decoded));
            }
            if let Ok(decoded) =
                <MigrationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::Migration(decoded));
            }
            if let Ok(decoded) =
                <MigrationHeightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::MigrationHeight(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RedeemVoucherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::RedeemVoucher(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferSelfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SafeTransferSelf(decoded));
            }
            if let Ok(decoded) =
                <SendToCosmosCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SendToCosmos(decoded));
            }
            if let Ok(decoded) =
                <SendToCronosCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SendToCronos(decoded));
            }
            if let Ok(decoded) =
                <SetAnyoneCanRelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SetAnyoneCanRelay(decoded));
            }
            if let Ok(decoded) =
                <StartMigrationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StartMigration(decoded));
            }
            if let Ok(decoded) =
                <StateRevertedVouchersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StateRevertedVouchers(decoded));
            }
            if let Ok(decoded) =
                <StateGravityIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StateGravityId(decoded));
            }
            if let Ok(decoded) =
                <StateInvalidationMappingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GravityCalls::StateInvalidationMapping(decoded));
            }
            if let Ok(decoded) =
                <StateLastBatchNoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StateLastBatchNonces(decoded));
            }
            if let Ok(decoded) =
                <StateLastEventNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StateLastEventNonce(decoded));
            }
            if let Ok(decoded) =
                <StateLastRevertedNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StateLastRevertedNonce(decoded));
            }
            if let Ok(decoded) =
                <StateLastValsetCheckpointCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GravityCalls::StateLastValsetCheckpoint(decoded));
            }
            if let Ok(decoded) =
                <StateLastValsetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StateLastValsetNonce(decoded));
            }
            if let Ok(decoded) =
                <StatePowerThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StatePowerThreshold(decoded));
            }
            if let Ok(decoded) =
                <StopMigrationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::StopMigration(decoded));
            }
            if let Ok(decoded) =
                <SubmitBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SubmitBatch(decoded));
            }
            if let Ok(decoded) =
                <SubmitLogicCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SubmitLogicCall(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <TestCheckValidatorSignaturesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GravityCalls::TestCheckValidatorSignatures(decoded));
            }
            if let Ok(decoded) =
                <TestMakeCheckpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::TestMakeCheckpoint(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferRelayerAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::TransferRelayerAdmin(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateValsetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GravityCalls::UpdateValset(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GravityCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GravityCalls::DefaultAdminRole(element) => element.encode(),
                GravityCalls::MigrationPeriod(element) => element.encode(),
                GravityCalls::Relayer(element) => element.encode(),
                GravityCalls::RelayerAdmin(element) => element.encode(),
                GravityCalls::AnyoneCanRelay(element) => element.encode(),
                GravityCalls::DeployERC20(element) => element.encode(),
                GravityCalls::GetRoleAdmin(element) => element.encode(),
                GravityCalls::GrantRole(element) => element.encode(),
                GravityCalls::HasRole(element) => element.encode(),
                GravityCalls::LastBatchNonce(element) => element.encode(),
                GravityCalls::LastLogicCallNonce(element) => element.encode(),
                GravityCalls::MigrateToken(element) => element.encode(),
                GravityCalls::Migration(element) => element.encode(),
                GravityCalls::MigrationHeight(element) => element.encode(),
                GravityCalls::Owner(element) => element.encode(),
                GravityCalls::Pause(element) => element.encode(),
                GravityCalls::Paused(element) => element.encode(),
                GravityCalls::RedeemVoucher(element) => element.encode(),
                GravityCalls::RenounceOwnership(element) => element.encode(),
                GravityCalls::RenounceRole(element) => element.encode(),
                GravityCalls::RevokeRole(element) => element.encode(),
                GravityCalls::SafeTransferSelf(element) => element.encode(),
                GravityCalls::SendToCosmos(element) => element.encode(),
                GravityCalls::SendToCronos(element) => element.encode(),
                GravityCalls::SetAnyoneCanRelay(element) => element.encode(),
                GravityCalls::StartMigration(element) => element.encode(),
                GravityCalls::StateRevertedVouchers(element) => element.encode(),
                GravityCalls::StateGravityId(element) => element.encode(),
                GravityCalls::StateInvalidationMapping(element) => element.encode(),
                GravityCalls::StateLastBatchNonces(element) => element.encode(),
                GravityCalls::StateLastEventNonce(element) => element.encode(),
                GravityCalls::StateLastRevertedNonce(element) => element.encode(),
                GravityCalls::StateLastValsetCheckpoint(element) => element.encode(),
                GravityCalls::StateLastValsetNonce(element) => element.encode(),
                GravityCalls::StatePowerThreshold(element) => element.encode(),
                GravityCalls::StopMigration(element) => element.encode(),
                GravityCalls::SubmitBatch(element) => element.encode(),
                GravityCalls::SubmitLogicCall(element) => element.encode(),
                GravityCalls::SupportsInterface(element) => element.encode(),
                GravityCalls::TestCheckValidatorSignatures(element) => element.encode(),
                GravityCalls::TestMakeCheckpoint(element) => element.encode(),
                GravityCalls::TransferOwnership(element) => element.encode(),
                GravityCalls::TransferRelayerAdmin(element) => element.encode(),
                GravityCalls::Unpause(element) => element.encode(),
                GravityCalls::UpdateValset(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GravityCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GravityCalls::DefaultAdminRole(element) => element.fmt(f),
                GravityCalls::MigrationPeriod(element) => element.fmt(f),
                GravityCalls::Relayer(element) => element.fmt(f),
                GravityCalls::RelayerAdmin(element) => element.fmt(f),
                GravityCalls::AnyoneCanRelay(element) => element.fmt(f),
                GravityCalls::DeployERC20(element) => element.fmt(f),
                GravityCalls::GetRoleAdmin(element) => element.fmt(f),
                GravityCalls::GrantRole(element) => element.fmt(f),
                GravityCalls::HasRole(element) => element.fmt(f),
                GravityCalls::LastBatchNonce(element) => element.fmt(f),
                GravityCalls::LastLogicCallNonce(element) => element.fmt(f),
                GravityCalls::MigrateToken(element) => element.fmt(f),
                GravityCalls::Migration(element) => element.fmt(f),
                GravityCalls::MigrationHeight(element) => element.fmt(f),
                GravityCalls::Owner(element) => element.fmt(f),
                GravityCalls::Pause(element) => element.fmt(f),
                GravityCalls::Paused(element) => element.fmt(f),
                GravityCalls::RedeemVoucher(element) => element.fmt(f),
                GravityCalls::RenounceOwnership(element) => element.fmt(f),
                GravityCalls::RenounceRole(element) => element.fmt(f),
                GravityCalls::RevokeRole(element) => element.fmt(f),
                GravityCalls::SafeTransferSelf(element) => element.fmt(f),
                GravityCalls::SendToCosmos(element) => element.fmt(f),
                GravityCalls::SendToCronos(element) => element.fmt(f),
                GravityCalls::SetAnyoneCanRelay(element) => element.fmt(f),
                GravityCalls::StartMigration(element) => element.fmt(f),
                GravityCalls::StateRevertedVouchers(element) => element.fmt(f),
                GravityCalls::StateGravityId(element) => element.fmt(f),
                GravityCalls::StateInvalidationMapping(element) => element.fmt(f),
                GravityCalls::StateLastBatchNonces(element) => element.fmt(f),
                GravityCalls::StateLastEventNonce(element) => element.fmt(f),
                GravityCalls::StateLastRevertedNonce(element) => element.fmt(f),
                GravityCalls::StateLastValsetCheckpoint(element) => element.fmt(f),
                GravityCalls::StateLastValsetNonce(element) => element.fmt(f),
                GravityCalls::StatePowerThreshold(element) => element.fmt(f),
                GravityCalls::StopMigration(element) => element.fmt(f),
                GravityCalls::SubmitBatch(element) => element.fmt(f),
                GravityCalls::SubmitLogicCall(element) => element.fmt(f),
                GravityCalls::SupportsInterface(element) => element.fmt(f),
                GravityCalls::TestCheckValidatorSignatures(element) => element.fmt(f),
                GravityCalls::TestMakeCheckpoint(element) => element.fmt(f),
                GravityCalls::TransferOwnership(element) => element.fmt(f),
                GravityCalls::TransferRelayerAdmin(element) => element.fmt(f),
                GravityCalls::Unpause(element) => element.fmt(f),
                GravityCalls::UpdateValset(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for GravityCalls {
        fn from(var: DefaultAdminRoleCall) -> Self {
            GravityCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<MigrationPeriodCall> for GravityCalls {
        fn from(var: MigrationPeriodCall) -> Self {
            GravityCalls::MigrationPeriod(var)
        }
    }
    impl ::std::convert::From<RelayerCall> for GravityCalls {
        fn from(var: RelayerCall) -> Self {
            GravityCalls::Relayer(var)
        }
    }
    impl ::std::convert::From<RelayerAdminCall> for GravityCalls {
        fn from(var: RelayerAdminCall) -> Self {
            GravityCalls::RelayerAdmin(var)
        }
    }
    impl ::std::convert::From<AnyoneCanRelayCall> for GravityCalls {
        fn from(var: AnyoneCanRelayCall) -> Self {
            GravityCalls::AnyoneCanRelay(var)
        }
    }
    impl ::std::convert::From<DeployERC20Call> for GravityCalls {
        fn from(var: DeployERC20Call) -> Self {
            GravityCalls::DeployERC20(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall> for GravityCalls {
        fn from(var: GetRoleAdminCall) -> Self {
            GravityCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for GravityCalls {
        fn from(var: GrantRoleCall) -> Self {
            GravityCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for GravityCalls {
        fn from(var: HasRoleCall) -> Self {
            GravityCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<LastBatchNonceCall> for GravityCalls {
        fn from(var: LastBatchNonceCall) -> Self {
            GravityCalls::LastBatchNonce(var)
        }
    }
    impl ::std::convert::From<LastLogicCallNonceCall> for GravityCalls {
        fn from(var: LastLogicCallNonceCall) -> Self {
            GravityCalls::LastLogicCallNonce(var)
        }
    }
    impl ::std::convert::From<MigrateTokenCall> for GravityCalls {
        fn from(var: MigrateTokenCall) -> Self {
            GravityCalls::MigrateToken(var)
        }
    }
    impl ::std::convert::From<MigrationCall> for GravityCalls {
        fn from(var: MigrationCall) -> Self {
            GravityCalls::Migration(var)
        }
    }
    impl ::std::convert::From<MigrationHeightCall> for GravityCalls {
        fn from(var: MigrationHeightCall) -> Self {
            GravityCalls::MigrationHeight(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for GravityCalls {
        fn from(var: OwnerCall) -> Self {
            GravityCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PauseCall> for GravityCalls {
        fn from(var: PauseCall) -> Self {
            GravityCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for GravityCalls {
        fn from(var: PausedCall) -> Self {
            GravityCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RedeemVoucherCall> for GravityCalls {
        fn from(var: RedeemVoucherCall) -> Self {
            GravityCalls::RedeemVoucher(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for GravityCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            GravityCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall> for GravityCalls {
        fn from(var: RenounceRoleCall) -> Self {
            GravityCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall> for GravityCalls {
        fn from(var: RevokeRoleCall) -> Self {
            GravityCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<SafeTransferSelfCall> for GravityCalls {
        fn from(var: SafeTransferSelfCall) -> Self {
            GravityCalls::SafeTransferSelf(var)
        }
    }
    impl ::std::convert::From<SendToCosmosCall> for GravityCalls {
        fn from(var: SendToCosmosCall) -> Self {
            GravityCalls::SendToCosmos(var)
        }
    }
    impl ::std::convert::From<SendToCronosCall> for GravityCalls {
        fn from(var: SendToCronosCall) -> Self {
            GravityCalls::SendToCronos(var)
        }
    }
    impl ::std::convert::From<SetAnyoneCanRelayCall> for GravityCalls {
        fn from(var: SetAnyoneCanRelayCall) -> Self {
            GravityCalls::SetAnyoneCanRelay(var)
        }
    }
    impl ::std::convert::From<StartMigrationCall> for GravityCalls {
        fn from(var: StartMigrationCall) -> Self {
            GravityCalls::StartMigration(var)
        }
    }
    impl ::std::convert::From<StateRevertedVouchersCall> for GravityCalls {
        fn from(var: StateRevertedVouchersCall) -> Self {
            GravityCalls::StateRevertedVouchers(var)
        }
    }
    impl ::std::convert::From<StateGravityIdCall> for GravityCalls {
        fn from(var: StateGravityIdCall) -> Self {
            GravityCalls::StateGravityId(var)
        }
    }
    impl ::std::convert::From<StateInvalidationMappingCall> for GravityCalls {
        fn from(var: StateInvalidationMappingCall) -> Self {
            GravityCalls::StateInvalidationMapping(var)
        }
    }
    impl ::std::convert::From<StateLastBatchNoncesCall> for GravityCalls {
        fn from(var: StateLastBatchNoncesCall) -> Self {
            GravityCalls::StateLastBatchNonces(var)
        }
    }
    impl ::std::convert::From<StateLastEventNonceCall> for GravityCalls {
        fn from(var: StateLastEventNonceCall) -> Self {
            GravityCalls::StateLastEventNonce(var)
        }
    }
    impl ::std::convert::From<StateLastRevertedNonceCall> for GravityCalls {
        fn from(var: StateLastRevertedNonceCall) -> Self {
            GravityCalls::StateLastRevertedNonce(var)
        }
    }
    impl ::std::convert::From<StateLastValsetCheckpointCall> for GravityCalls {
        fn from(var: StateLastValsetCheckpointCall) -> Self {
            GravityCalls::StateLastValsetCheckpoint(var)
        }
    }
    impl ::std::convert::From<StateLastValsetNonceCall> for GravityCalls {
        fn from(var: StateLastValsetNonceCall) -> Self {
            GravityCalls::StateLastValsetNonce(var)
        }
    }
    impl ::std::convert::From<StatePowerThresholdCall> for GravityCalls {
        fn from(var: StatePowerThresholdCall) -> Self {
            GravityCalls::StatePowerThreshold(var)
        }
    }
    impl ::std::convert::From<StopMigrationCall> for GravityCalls {
        fn from(var: StopMigrationCall) -> Self {
            GravityCalls::StopMigration(var)
        }
    }
    impl ::std::convert::From<SubmitBatchCall> for GravityCalls {
        fn from(var: SubmitBatchCall) -> Self {
            GravityCalls::SubmitBatch(var)
        }
    }
    impl ::std::convert::From<SubmitLogicCallCall> for GravityCalls {
        fn from(var: SubmitLogicCallCall) -> Self {
            GravityCalls::SubmitLogicCall(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for GravityCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            GravityCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<TestCheckValidatorSignaturesCall> for GravityCalls {
        fn from(var: TestCheckValidatorSignaturesCall) -> Self {
            GravityCalls::TestCheckValidatorSignatures(var)
        }
    }
    impl ::std::convert::From<TestMakeCheckpointCall> for GravityCalls {
        fn from(var: TestMakeCheckpointCall) -> Self {
            GravityCalls::TestMakeCheckpoint(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for GravityCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            GravityCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TransferRelayerAdminCall> for GravityCalls {
        fn from(var: TransferRelayerAdminCall) -> Self {
            GravityCalls::TransferRelayerAdmin(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for GravityCalls {
        fn from(var: UnpauseCall) -> Self {
            GravityCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UpdateValsetCall> for GravityCalls {
        fn from(var: UpdateValsetCall) -> Self {
            GravityCalls::UpdateValset(var)
        }
    }
    #[doc = "Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `MIGRATION_PERIOD` function with signature `MIGRATION_PERIOD()` and selector `[40, 183, 199, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct MigrationPeriodReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `RELAYER` function with signature `RELAYER()` and selector `[36, 131, 231, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct RelayerReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `RELAYER_ADMIN` function with signature `RELAYER_ADMIN()` and selector `[169, 124, 207, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct RelayerAdminReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `anyoneCanRelay` function with signature `anyoneCanRelay()` and selector `[60, 3, 39, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct AnyoneCanRelayReturn(pub bool);
    #[doc = "Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct HasRoleReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastBatchNonce` function with signature `lastBatchNonce(address)` and selector `[1, 27, 33, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct LastBatchNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `lastLogicCallNonce` function with signature `lastLogicCallNonce(bytes32)` and selector `[201, 209, 148, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct LastLogicCallNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `migration` function with signature `migration()` and selector `[23, 5, 163, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct MigrationReturn(pub bool);
    #[doc = "Container type for all return fields from the `migrationHeight` function with signature `migrationHeight()` and selector `[96, 150, 158, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct MigrationHeightReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct PausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `state_RevertedVouchers` function with signature `state_RevertedVouchers(uint256)` and selector `[125, 85, 24, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateRevertedVouchersReturn {
        pub token_contract: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `state_gravityId` function with signature `state_gravityId()` and selector `[189, 218, 129, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateGravityIdReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `state_invalidationMapping` function with signature `state_invalidationMapping(bytes32)` and selector `[125, 251, 111, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateInvalidationMappingReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `state_lastBatchNonces` function with signature `state_lastBatchNonces(address)` and selector `[223, 151, 23, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateLastBatchNoncesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `state_lastEventNonce` function with signature `state_lastEventNonce()` and selector `[115, 178, 5, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateLastEventNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `state_lastRevertedNonce` function with signature `state_lastRevertedNonce()` and selector `[241, 109, 103, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateLastRevertedNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `state_lastValsetCheckpoint` function with signature `state_lastValsetCheckpoint()` and selector `[242, 181, 51, 7]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateLastValsetCheckpointReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `state_lastValsetNonce` function with signature `state_lastValsetNonce()` and selector `[181, 101, 97, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StateLastValsetNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `state_powerThreshold` function with signature `state_powerThreshold()` and selector `[229, 162, 181, 210]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct StatePowerThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "`LogicCallArgs(uint256[],address[],uint256[],address[],address,bytes,uint256,bytes32,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct LogicCallArgs {
        pub transfer_amounts: Vec<ethers::core::types::U256>,
        pub transfer_token_contracts: Vec<ethers::core::types::Address>,
        pub fee_amounts: Vec<ethers::core::types::U256>,
        pub fee_token_contracts: Vec<ethers::core::types::Address>,
        pub logic_contract_address: ethers::core::types::Address,
        pub payload: ethers::core::types::Bytes,
        pub time_out: ethers::core::types::U256,
        pub invalidation_id: [u8; 32],
        pub invalidation_nonce: ethers::core::types::U256,
    }
    #[doc = "`PaymentArgs(uint256[],address[],uint256[],address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct PaymentArgs {
        pub amounts: Vec<ethers::core::types::U256>,
        pub destinations: Vec<ethers::core::types::Address>,
        pub fees: Vec<ethers::core::types::U256>,
        pub fee_payment_address: ethers::core::types::Address,
    }
    #[doc = "`ValSignature(uint8,bytes32,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct ValSignature {
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "`ValsetArgs(address[],uint256[],uint256,uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct ValsetArgs {
        pub validators: Vec<ethers::core::types::Address>,
        pub powers: Vec<ethers::core::types::U256>,
        pub valset_nonce: ethers::core::types::U256,
        pub reward_amount: ethers::core::types::U256,
        pub reward_token: ethers::core::types::Address,
    }
}
