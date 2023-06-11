#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::oracle_contract::OracleContractRef;

#[ink::contract]
mod oracle_contract {
    use ink::prelude::vec::Vec;
    use ink::storage::{
        Lazy,
        Mapping,
        traits::ManualKey,
    };

    /// A block number.
    pub type BlockNumber = u64;
    pub type MarketGuessId = String;
    pub type OracleOwner = AccountId;
    pub type BlockNumberEntropy = BlockNumber;

    pub enum Status {
        Pending,
        Finalized,
    }

    pub struct MarketGuess {
        /// Market guess id.
        id_market: MarketGuessId,
        /// Block number when market guesses were made.
        block_number_guessed: BlockNumber,
        /// Block number in the future to use the block hash of for entropy.
        block_number_entropy: BlockNumber,
        /// Block hash associated with `block_number_entropy` when finalized
        /// to use for entropy.
        block_hash_entropy: Hash,
        /// Market guess period end block number
        block_number_end: BlockNumber,
        /// Market guess finalization status.
        status: Status,
    }

    #[derive(Default)]
    #[ink(storage)]
    pub struct OracleContract {
        /// Assign an owner and block number for entropy to every market guess id.
        market_data: Mapping<MarketGuessId, (OracleOwner, BlockNumberEntropy), ManualKey<123>>,
        /// Enforce non-`Packed` storage layout since vector may get large
        /// and expensive to work with.
        /// Lazy fields like `market_guesses` provide `get()` and `set()`
        /// storage operators.
        market_guesses: Lazy<Vec<MarketGuess>>,
    }

    impl OracleContract {
        #[ink(constructor)]
        pub fn new(
            id_market: MarketGuessId,
            block_number_guessed: BlockNumber,
            block_number_entropy: BlockNumber,
            block_number_end: BlockNumber,
        ) -> Self {
            let mut instance = Self::default();
            let caller: OracleOwner = Self.env().caller();
            let market_data = Mapping::default();
            assert!(self.exists_market_data_for_id, "unable to find market data for given id");
            let block_number_current: Self::env().block_number();
            // TODO - we need to verify that the block hash exists for the block number
            // when they say guessing occurred
            assert!(
                block_number_current > block_number_guessed,
                "block number when guessing occurred must be before the current block number"
            );
            // TODO - 100 is a magic number
            assert!(
                block_number_entropy - block_number_current > 100,
                "block used for entropy must allow sufficient block confirmations after the current \
                block and block when guessing occurred for assurance that epoch gets finalized \
                incase head susceptible to reorganization for safety"
            );
            // TODO - 200 is a magic number
            assert!(
                block_number_end - block_number_entropy > 200,
                "block when market ends must be allow sufficient block confirmations after the \
                block used for entropy"
            );

            instance.market_data.insert(&id_market, (&caller, &block_number_entropy)));
            let new_market_guess = MarketGuess {
                id_market,
                block_number_guessed,
                block_number_entropy,
                block_hash_entropy: Default::default(),
                block_number_end,
                status: Status::Pending,
            };
            let mut market_guesses = self.market_guesses.get_or_default();
            market_guesses.push(new_market_guess);
            self.market_guesses.set(&market_guesses);
            instance
        }

        // #[ink(message)]
        // pub fn set_block_number_guessed(&mut self, block_number) {
        //     self.block_number_guessed = block_number;
        // }
        // #[ink(message)]
        // pub fn get_block_number_guessed(&self) -> BlockNumber {
        //     self.block_number_guessed
        // }
        // assert!(caller == owner, "caller is not owner")

        #[ink(message)]
        pub fn set_block_hash_entropy_for_market_id(
            &mut self,
            id_market: MarketGuessId,
            block_hash_entropy: Hash,
        ) {
            self.block_number_guessed = block_number;
        }

        #[ink(message)]
        pub fn get_block_hash_entropy_for_market_id(&self, id_market) -> BlockNumber {
            self.block_number_guessed
        }

        // helper methods
        fn exists_market_data_for_id(id_market: MarketGuessId) -> bool {
            self.market_data.get(id_market).is_some()
        }
    }
}
