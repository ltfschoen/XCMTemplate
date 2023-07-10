// SPDX-License-Identifier: MIT
pragma solidity >=0.8.3;

import {RandomNumber} from "./lib/RandomNumber.sol";

contract FlipperGame {
    uint256 private constant BLOCKS_ALLOW_GUESS = 100;
    uint public blockNumber;
    // bytes32 public blockHashNow;
    bytes32 public blockHashPrevious;
    bool public hasCalledFallbackFn;
    address public s_owner;
    address public randomNumberContractAddress;
    uint256 public gameId = 0;
    uint256 public playerGuessId = 0;
    uint256 public answerId = 0;
    GameStruct[] public gamesList;

    struct GameStruct {
        uint256 id;
        bytes32 startGameAtBlockHash; // blockHashPrevious
        address createdBy;
        uint createdAtBlock;
    }

    struct PlayerStruct {
        address playerAddress;
        uint256 gameId;
        uint createdAtBlock;
    }

    struct PlayerGuessStruct {
        uint256 id;
        uint256 gameId;
        address guessByPlayerAddress;
        uint8 guess;
        uint createdAtBlock;
    }

    struct AnswerStruct {
        uint256 id;
        uint256 gameId;
        uint8 answer;
        uint createdAtBlock;
    }

    mapping(address => uint) public playerBalance;
    mapping(uint256 => GameStruct) public gameForGameId;
    mapping(uint256 => PlayerStruct[]) public playersForGameId;
    mapping(address => uint256[]) public gameIdsForPlayerId;
    mapping(uint256 => PlayerGuessStruct[]) public playerGuessForPlayerOfGameId;
    mapping(uint256 => AnswerStruct) public answerForGameId;

    event CreatedGame(uint256 gameId, bytes32 blockHashPrevious, address createdBy,
        uint createdAtBlock, uint endGuessesAtBlockNumber);
    event AddedPlayerToGame(uint256 gameId, address playerAddress, uint createdAtBlock);
    event AddedGuessForPlayerOfGame(uint256 gameId, uint256 playerGuessId,
        address guessByPlayerAddress, uint8 guess, uint createdAtBlock);
    event AddedAnswerForGame(uint256 gameId, uint256 answerId, address answerByAddress,
        uint8 answer, uint createdAtBlock);

    constructor() {
        s_owner = msg.sender;
    }

    function createGame(uint8 _initialGuess) external payable returns(uint256) {
        // Make sure that the value sent is enough
        require(msg.value >= RandomNumber.MIN_FEE, "Insufficient fulfillment fee");
        require(_initialGuess <= 20);
        blockNumber = block.number;
        blockHashPrevious = blockhash(blockNumber - 1);

        gameId = gameId + 1;
        uint256 _gameId = gameId;

        GameStruct memory gameInstance = GameStruct({
            id: _gameId,
            startGameAtBlockHash: blockHashPrevious,
            createdBy: msg.sender,
            createdAtBlock: blockNumber,
            endGuessesAtBlockNumber: blockNumber + BLOCKS_ALLOW_GUESS
        });

        gameForGameId[_gameId] = gameInstance;
        gamesList.push(gameInstance);    

        emit CreatedGame(_gameId, blockHashPrevious, msg.sender, blockNumber, blockNumber + BLOCKS_ALLOW_GUESS);

        return _gameId;
    }

    function addPlayerToGame(uint256 _gameId, uint8 _initialGuess) payable public {
        blockNumber = block.number;
        require(hasPlayerForGameId(_gameId) == false, "only one instance of same player address per game");
        require(blockNumber >= gameForGameId[_gameId].createdAtBlock);
        require(blockNumber <= gameForGameId[_gameId].endGuessesAtBlockNumber,
            "add player to game only allowed before block number when guesses end");
        PlayerStruct memory playerInstance = PlayerStruct({
            playerAddress: msg.sender,
            gameId: _gameId,
            createdAtBlock: blockNumber
        });
        playersForGameId[_gameId].push(playerInstance);
        gameIdsForPlayerId[msg.sender].push(_gameId);

        addGuessForPlayerOfGame(_gameId, _initialGuess);

        emit AddedPlayerToGame(_gameId, msg.sender, blockNumber);
    }

    function hasPlayerForGameId(uint256 _gameId) public view returns (bool) {
        for (uint i = 0; i < playersForGameId[_gameId].length; i++) {
            if (_gameId[_gameId][i] == msg.sender) {
                return true;
            }
        }
        return false;
    }

    function hasPlayerGuessedForGameId(uint256 _gameId) public view returns (bool) {
        for (uint i = 0; i < gameIdsForPlayerId[msg.sender].length; i++) {
            if (gameIdsForPlayerId[msg.sender][i] == _gameId) {
                return true;
            }
        }
        return false;
    }

    function addGuessForPlayerOfGame(uint256 _gameId, uint8 _guess) internal returns(uint8) {
        require(_guess <= 20);
        require(hasPlayerGuessedForGameId(_gameId) == false, "only one guess per player per game");
        blockNumber = block.number;
        require(blockNumber >= gameForGameId[_gameId].createdAtBlock);
        require(blockNumber <= gameForGameId[_gameId].endGuessesAtBlockNumber,
            "guesses only allowed before block number when guesses end");
        playerGuessId = playerGuessId + 1;
        uint256 _playerGuessId = playerGuessId;
        PlayerGuessStruct memory playerGuessInstance = PlayerGuessStruct({
            id: _playerGuessId,
            gameId: _gameId,
            guessByPlayerAddress: msg.sender,
            guess: _guess,
            createdAtBlock: blockNumber
        });
        playerGuessForPlayerOfGameId[_gameId].push(playerGuessInstance);
        gameIdsForPlayerId[msg.sender].push(_gameId);

        emit AddedGuessForPlayerOfGame(_gameId, _playerGuessId, msg.sender, _guess, blockNumber);

        return _guess;
    }

    function addAnswerToGame(uint256 _gameId) external returns(uint8) {
        blockNumber = block.number;
        require(blockNumber >= gameForGameId[_gameId].createdAtBlock);
        require(blockNumber > gameForGameId[_gameId].endGuessesAtBlockNumber,
            "answer only allowed after block number when guesses end");

        // TODO

        require(answer <= 20);

        emit AddedAnswerForGame(_gameId, answerId, msg.sender, answer, blockNumber);
    }

    function setRandomNumberContractAddress(address _randomNumberAddress)
        public onlyOwner payable
    {
        randomNumberContractAddress = _randomNumberAddress;
    }

    // fallback function
    fallback() external payable {
        hasCalledFallbackFn = true;
        playerBalance[msg.sender] += msg.value;
    }
    receive() external payable {}

    modifier onlyOwner() {
        require(msg.sender == s_owner);
        _;
    }
}
