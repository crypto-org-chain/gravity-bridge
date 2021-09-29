pragma solidity ^0.6.6;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@uniswap/v2-periphery/contracts/interfaces/IWETH.sol";
import "./Gravity.sol";

contract EthGravityWrapper {
	address immutable WETH_ADDRESS;
	address immutable GRAVITY_ADDRESS;
	uint256 constant MAX_VALUE = uint256(-1);

	event SendToCosmosEthEvent(
		address indexed _sender,
		bytes32 indexed _destination,
		uint256 _amount
	);

	constructor(address _wethAddress, address _gravityAddress) public {
		WETH_ADDRESS = _wethAddress;
		GRAVITY_ADDRESS = _gravityAddress;
 
		IERC20(_wethAddress).approve(_gravityAddress, MAX_VALUE);
	}

	function sendToCosmosEth(bytes32 _destination) public payable {
		uint256 amount = msg.value;
		require(amount > 0, "Amount should be greater than 0");

		IWETH(WETH_ADDRESS).deposit{ value: amount }();
		Gravity(GRAVITY_ADDRESS).sendToCosmos(WETH_ADDRESS, _destination, amount);

		emit SendToCosmosEthEvent(msg.sender, _destination, amount);
	}
}
