pragma solidity 0.8.10;

import "./Gravity.sol";

interface IWETH {
	function deposit() external payable;
	function approve(address spender, uint256 amount) external;
}

contract EthGravityWrapper {
	IWETH public immutable weth;
	Gravity public immutable gravity;

	uint256 constant MAX_VALUE = type(uint256).max;

	event sendToCronosEthEvent(
		address indexed _sender,
		address indexed _destination,
		uint256 _amount
	);

	constructor(address _wethAddress, address _gravityAddress) public {
		weth = IWETH(_wethAddress);
		gravity = Gravity(_gravityAddress);

		IWETH(_wethAddress).approve(_gravityAddress, MAX_VALUE);
	}

	function sendToCronosEth(address _destination) public payable {
		uint256 amount = msg.value;
		require(amount > 0, "Amount should be greater than 0");

		weth.deposit{ value: amount }();
		gravity.sendToCronos(address(weth), _destination, amount);

		emit sendToCronosEthEvent(msg.sender, _destination, amount);
	}
}
