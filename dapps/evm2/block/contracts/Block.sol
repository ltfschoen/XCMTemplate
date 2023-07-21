// SPDX-License-Identifier: MIT
pragma solidity >=0.8.3;

contract Block {
    address public s_owner;

    // event PreviousBlockHashByOffset(
    //     uint256 indexed blockNumber,
    //     address indexed requestedBy,
    //     bytes32 previousBlockHash
    // );

    constructor() {
        s_owner = msg.sender;
    }

    function getPreviousBlockHashByOffset(uint256 currentBlockNumberFromCaller, uint256 offset) public view returns (bytes32) {
        uint256 blockNumber = block.number;
        require(blockNumber == currentBlockNumberFromCaller, "current block number from caller is not same as current block number in called function");
        bytes32 previousBlockHash = blockhash(currentBlockNumberFromCaller - offset);
        // emit PreviousBlockHashByOffset(blockNumber, msg.sender, previousBlockHash);
        return previousBlockHash;
    }
}
