pragma solidity ^0.6.6;

import "./Gravity.sol";

interface IWETH {
	function deposit() external payable;
	function approve(address spender, uint256 amount) external;
}

contract EthGravityWrapper {
	IWETH public immutable weth;
	Gravity public immutable gravity;

	uint256 constant MAX_VALUE = uint256(-1);

	event SendToCosmosEthEvent(
		address indexed _sender,
		bytes32 indexed _destination,
		uint256 _amount
	);

	constructor(address _wethAddress, address _gravityAddress) public {
		weth = IWETH(_wethAddress);
		gravity = Gravity(_gravityAddress);

		IWETH(_wethAddress).approve(_gravityAddress, MAX_VALUE);
	}

	function sendToCosmosEth(bytes32 _destination) public payable {
		uint256 amount = msg.value;
		require(amount > 0, "Amount should be greater than 0");

		weth.deposit{ value: amount }();
		gravity.sendToCosmos(address(weth), _destination, amount);

		emit SendToCosmosEthEvent(msg.sender, _destination, amount);
	}
}
