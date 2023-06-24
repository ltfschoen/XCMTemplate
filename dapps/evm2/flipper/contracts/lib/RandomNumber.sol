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
    uint256 public MIN_FEE = FULFILLMENT_GAS_LIMIT * 5 gwei;
    uint32 public VRF_BLOCKS_DELAY;
    bytes32 public SALT_PREFIX = "change-me-to-anything";

    // Storage variables for the current request
    uint256 public requestId;
    uint256[] public random;

    constructor() payable RandomnessConsumer() {
        // Initialize use of Randomness dependency before trying to access it
        theRandomness = Randomness(randomnessPrecompileAddress);
        requiredDeposit = theRandomness.requiredDeposit();
        // Because this contract can only perform 1 random request at a time,
        // We only need to have 1 required deposit.
        // require(msg.value >= requiredDeposit);
        VRF_BLOCKS_DELAY = MIN_VRF_BLOCKS_DELAY;
    }

    function requestRandomness() public payable {
        // Make sure that the value sent is enough
        require(msg.value >= MIN_FEE);
        // Request local VRF randomness
        requestId = theRandomness.requestLocalVRFRandomWords(
            msg.sender, // Refund address
            msg.value, // Fulfillment fee
            FULFILLMENT_GAS_LIMIT, // Gas limit for the fulfillment
            SALT_PREFIX ^ bytes32(requestId++), // A salt to generate unique results
            1, // Number of random words
            VRF_BLOCKS_DELAY // Delay before request can be fulfilled
        );
    }

    function getRequestStatus() public view returns(Randomness.RequestStatus) {
        Randomness.RequestStatus requestStatus = theRandomness.getRequestStatus(requestId);
        return requestStatus;
    }

    function fulfillRequest() public {
        theRandomness.fulfillRequest(requestId);
    }

    function fulfillRandomWords(
        uint256, /* requestId */
        uint256[] memory randomWords
    ) internal override {
        // Save the randomness results
        random = randomWords;
    }
}
