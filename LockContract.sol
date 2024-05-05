// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

//import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract LockContract {
    // Event emitted upon locking ERC20 tokens
    event LockERC20(address indexed sender, address indexed token, uint256 amount, string targetChain, string targetAddress);

    // Event emitted upon locking native ETH
    event LockETH(address indexed sender, uint256 amount, string targetChain, string targetAddress);

    // Lock ERC20 tokens
    function lockTokens(address token, uint256 amount, string memory targetChain, string memory targetAddress) external {
        require(amount > 0, "Amount must be greater than zero");
        require(bytes(targetChain).length > 0, "Target chain must be specified");
        require(bytes(targetAddress).length > 0, "Target address must be specified");

        // Transfer tokens from sender to this contract
        IERC20(token).transferFrom(msg.sender, address(this), amount);

        // Emit lock event
        emit LockERC20(msg.sender, token, amount, targetChain, targetAddress);
    }

    // Lock native ETH
    function lockETH(string memory targetChain, string memory targetAddress) external payable {
        require(msg.value > 0, "ETH amount must be greater than zero");
        require(bytes(targetChain).length > 0, "Target chain must be specified");
        require(bytes(targetAddress).length > 0, "Target address must be specified");

        // Emit lock event
        emit LockETH(msg.sender, msg.value, targetChain, targetAddress);
    }

    // Fallback function to receive ETH
    receive() external payable {}
}
