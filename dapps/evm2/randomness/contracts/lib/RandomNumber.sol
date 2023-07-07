// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

// compiling with truffle automatically generates the respective ABI files for these precompiles
// https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol
// import "../precompiles/randomness/Randomness.sol";
import {Randomness, MIN_VRF_BLOCKS_DELAY} from "../precompiles/randomness/Randomness.sol";
// https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/RandomnessConsumer.sol
import {RandomnessConsumer} from "../precompiles/randomness/RandomnessConsumer.sol";

contract RandomNumber is RandomnessConsumer {
    // The Randomness Precompile Interface

    // create a wrapper to access the randomness precompile
    Randomness public theRandomness;
    address public constant randomnessPrecompileAddress = 0x0000000000000000000000000000000000000809;

    // Variables required for randomness requests
    uint256 public requiredDeposit;
    uint64 public FULFILLMENT_GAS_LIMIT = 100000;
    // The fee can be set to any value as long as it is enough to cover
    // the fulfillment costs. Any leftover fees will be refunded to the
    // refund address specified in the requestRandomness function below
    uint256 public MIN_FEE = FULFILLMENT_GAS_LIMIT * 5 gwei; // 0.000000001 Ether == 1 gwei
    uint32 public VRF_BLOCKS_DELAY;
    bytes32 public SALT_PREFIX = "change-me-to-anything";

    uint256 private constant ROLL_IN_PROGRESS = 42;

    // Storage variables for the current request
    uint256 public requestId;
    uint256[] public random;

    // address public s_owner;

    // map rollers to requestIds
    mapping(uint256 => address) private s_rollers;
    // map vrf results to rollers
    mapping(address => uint256) private s_results;

    event DiceRolled(uint256 indexed requestId, address indexed roller);
    event DiceLanded(uint256 indexed requestId, uint256 indexed result);

    constructor() payable RandomnessConsumer() {
        // Initialize use of Randomness dependency before trying to access it
        theRandomness = Randomness(randomnessPrecompileAddress);
        requiredDeposit = theRandomness.requiredDeposit();
        // Because this contract can only perform 1 random request at a time,
        // We only need to have 1 required deposit.
        require(msg.value >= requiredDeposit);
        // s_owner = msg.sender;
        VRF_BLOCKS_DELAY = MIN_VRF_BLOCKS_DELAY;
    }

    function requestRandomness(
        address roller
    ) public payable {
    // ) public onlyOwner payable {
        require(s_results[roller] == 0, "Already rolled");
        // Make sure that the value sent is enough
        require(msg.value >= MIN_FEE, "Insufficient fulfillment fee");
        // Request local VRF randomness
        requestId = theRandomness.requestRelayBabeEpochRandomWords(
            msg.sender, // Refund address
            msg.value, // Fulfillment fee
            FULFILLMENT_GAS_LIMIT, // Gas limit for the fulfillment
            SALT_PREFIX ^ bytes32(requestId++), // A salt to generate unique results
            1 // Number of random words
            // 1, // Number of random words
            // VRF_BLOCKS_DELAY // Delay before request can be fulfilled
        );

        s_rollers[requestId] = roller;
        s_results[roller] = ROLL_IN_PROGRESS;
        emit DiceRolled(requestId, roller);
    }

    function getRequestStatus() public view returns(Randomness.RequestStatus) {
        Randomness.RequestStatus requestStatus = theRandomness.getRequestStatus(requestId);
        return requestStatus;
    }

    function fulfillRequest() public {
        theRandomness.fulfillRequest(requestId);
    }

    function fulfillRandomWords(
        uint256 reqId, /* requestId */
        uint256[] memory randomWords
    ) internal override {
        // Save the randomness results
        uint256 d20Value = (randomWords[0] % 20) + 1;
        s_results[s_rollers[reqId]] = d20Value;
        // Save the latest for comparison
        random = randomWords;
    }

    /**
     * @notice Get the value rolled by a player once the address has rolled
     * @param player address
     * @return id as a string
     */
    function getRolledValueForPlayer(address player) public view returns (uint256) {
        require(s_results[player] != 0, "Dice not rolled");
        require(s_results[player] != ROLL_IN_PROGRESS, "Roll in progress");
        return s_results[player];
    }

    // modifier onlyOwner() {
    //     require(msg.sender == s_owner);
    //     _;
    // }
}
