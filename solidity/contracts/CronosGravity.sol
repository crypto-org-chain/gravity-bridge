pragma solidity ^0.6.6;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "./Gravity.sol";

pragma experimental ABIEncoderV2;

contract CronosGravity is Gravity, AccessControl, Pausable, Ownable {
    bytes32 public constant RELAYER = keccak256("RELAYER");
    bytes32 public constant RELAYER_ADMIN = keccak256("RELAYER_ADMIN");

    modifier onlyRole(bytes32 role) {
        require(hasRole(role, msg.sender), "CronosGravity::Permission Denied");
        _;
    }

    constructor (
        bytes32 _gravityId,
        uint256 _powerThreshold,
        address[] memory _validators,
        uint256[] memory _powers,
        address relayerAdmin
    ) public Gravity(
        _gravityId,
        _powerThreshold,
        _validators,
        _powers
    ) {
        _setupRole(RELAYER_ADMIN, relayerAdmin);
        _setRoleAdmin(RELAYER, RELAYER_ADMIN);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function updateValset(
        // The new version of the validator set
        address[] memory _newValidators,
        uint256[] memory _newPowers,
        uint256 _newValsetNonce,
        // The current validators that approve the change
        address[] memory _currentValidators,
        uint256[] memory _currentPowers,
        uint256 _currentValsetNonce,
        // These are arrays of the parts of the current validator's signatures
        uint8[] memory _v,
        bytes32[] memory _r,
        bytes32[] memory _s
    ) public override whenNotPaused onlyRole(RELAYER) {
        super.updateValset(
            _newValidators, _newPowers, _newValsetNonce,
            _currentValidators, _currentPowers, _currentValsetNonce,
            _v, _r, _s
        );
    }

    function submitBatch(
        // The validators that approve the batch
        address[] memory _currentValidators,
        uint256[] memory _currentPowers,
        uint256 _currentValsetNonce,
        // These are arrays of the parts of the validators signatures
        uint8[] memory _v,
        bytes32[] memory _r,
        bytes32[] memory _s,
        // The batch of transactions
        uint256[] memory _amounts,
        address[] memory _destinations,
        uint256[] memory _fees,
        uint256 _batchNonce,
        address _tokenContract,
        // a block height beyond which this batch is not valid
        // used to provide a fee-free timeout
        uint256 _batchTimeout
    ) public override whenNotPaused onlyRole(RELAYER) {
        super.submitBatch(
            _currentValidators, _currentPowers, _currentValsetNonce,
            _v, _r,  _s,
            _amounts, _destinations, _fees, _batchNonce, _tokenContract,
            _batchTimeout
        );
    }

    function submitLogicCall(
        // The validators that approve the call
        address[] memory _currentValidators,
        uint256[] memory _currentPowers,
        uint256 _currentValsetNonce,
        // These are arrays of the parts of the validators signatures
        uint8[] memory _v,
        bytes32[] memory _r,
        bytes32[] memory _s,
        LogicCallArgs memory _args
    ) public override whenNotPaused onlyRole(RELAYER) {
        super.submitLogicCall(
            _currentValidators, _currentPowers, _currentValsetNonce,
            _v, _r, _s,
            _args
        );
    }

    function sendToCronos(
        address _tokenContract,
        address _destination,
        uint256 _amount
    ) public override whenNotPaused {
        super.sendToCronos(
            _tokenContract, _destination, _amount
        );
    }
}
