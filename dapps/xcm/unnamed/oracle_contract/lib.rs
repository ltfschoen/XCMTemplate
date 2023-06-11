#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::oracle_contract::OracleContractRef;

#[ink::contract]
mod oracle_contract {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::{
        Lazy,
        Mapping,
        // traits::ManualKey,
    };
    use ink::storage::traits::{
        StorableHint,
        StorageKey,
        Storable,
    };
    // use core::str;
    // use hex;

    // refactor into types file
    pub type MarketGuessId = Vec<u8>;
    pub type OracleOwner = AccountId;

    /// Emitted when create new market guess for market id.
    #[ink(event)]
    pub struct NewOracleMarketGuessForMarketId {
        #[ink(topic)]
        id_market: MarketGuessId,
        #[ink(topic)]
        oracle_owner: OracleOwner,
        #[ink(topic)]
        block_number_guessed: BlockNumber,
        #[ink(topic)]
        block_number_entropy: BlockNumber,
        // #[ink(topic)]
        block_number_end: BlockNumber,
    }

    /// Emitted when set block hash entropy for market id.
    #[ink(event)]
    pub struct SetBlockHashEntropyForMarketId {
        #[ink(topic)]
        id_market: MarketGuessId,
        oracle_owner: OracleOwner,
        #[ink(topic)]
        block_number_entropy: BlockNumber,
        #[ink(topic)]
        block_hash_entropy: Hash,
    }

    // https://docs.rs/ink/latest/ink/attr.storage_item.html
    // Non-packed types like `Mapping` require calculating the storage key during compilation
    // and it is best to rely on automatic storage key calculation via `ink::storage_item`
    // and must be called before `derive`
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    #[derive(Default, Debug)]
    pub struct MarketGuess {
        /// Market guess id.
        id_market: MarketGuessId,
        /// Oracle account owner
        oracle_owner: Option<OracleOwner>,
        /// Block number when market guesses were made.
        block_number_guessed: BlockNumber,
        /// Block number in the future to use the block hash of for entropy.
        block_number_entropy: BlockNumber,
        /// Block hash associated with `block_number_entropy` when finalized
        /// to use for entropy.
        block_hash_entropy: Hash,
        /// Market guess period end block number
        block_number_end: BlockNumber,
    }

    #[derive(Default)]
    #[ink(storage)]
    pub struct OracleContract {
        /// Assign an owner and block number for entropy to every market guess id.
        market_data: Mapping<MarketGuessId, MarketGuess>, // , ManualKey<123>
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the no data exists for given market guess id.
        NoDataForMarketGuessId,
        /// Returned if caller is not oracle owner of market guess id.
        CallerIsNotOracleOwner,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl OracleContract {
        #[ink(constructor)]
        pub fn new(
            id_market: MarketGuessId,
            block_number_guessed: BlockNumber,
            block_number_entropy: BlockNumber,
            block_number_end: BlockNumber,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            assert!(instance.exists_market_data_for_id(id_market.clone()), "unable to find market data for given id");
            let block_number_current = Self::env().block_number();
            // TODO - we need to verify that the block hash exists for the block number
            // when they say guessing occurred
            assert!(
                block_number_current > block_number_guessed,
                "block number when guessing occurred must be before the current block number"
            );
            // TODO - 100 and 200 are magic numbers, need something more meaningful
            assert!(
                block_number_entropy - block_number_current > 100,
                "block used for entropy must allow sufficient block confirmations after the current \
                block and block when guessing occurred for assurance that epoch gets finalized \
                incase head susceptible to reorganization for safety"
            );
            assert!(
                block_number_end - block_number_entropy > 200,
                "block when market ends must be allow sufficient block confirmations after the \
                block used for entropy"
            );
            let new_market_guess = MarketGuess {
                id_market: id_market.clone(),
                // must be set to Option<AccountId> to avoid error:
                // the trait `Default` is not implemented for `ink::ink_primitives::AccountId`
                oracle_owner: Some(caller),
                block_number_guessed,
                block_number_entropy,
                block_hash_entropy: Default::default(),
                block_number_end,
            };
            instance.market_data.insert(&id_market, &new_market_guess);
            instance.env().emit_event(NewOracleMarketGuessForMarketId {
                id_market: id_market.clone(),
                oracle_owner: caller,
                block_number_guessed,
                block_number_entropy,
                block_number_end,
            });
            instance
        }

        #[ink(message)]
        pub fn set_block_hash_entropy_for_market_id(
            &mut self,
            id_market: MarketGuessId,
            block_hash_entropy: Hash,
        ) -> Result<()> {
            let caller: AccountId = self.env().caller();
            // assert!(self.exists_market_data_for_id(id_market), "unable to find market data for given id");
            // TODO - convert Vec<u8> to &str to avoid use of .clone()
            let market_guess = match self.market_data.get(id_market.clone()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            // singleton change of block hash entropy from the default value set at instantiation of the contract
            assert!(market_guess.block_hash_entropy == Default::default(), "unable to set block hash entropy for market id once");
            if market_guess.oracle_owner != Some(caller) {
                return Err(Error::CallerIsNotOracleOwner)
            }
            let new_market_guess = MarketGuess {
                block_hash_entropy,
                ..market_guess
            };
            self.market_data.insert(id_market.clone(), &new_market_guess);
            self.env().emit_event(SetBlockHashEntropyForMarketId {
                id_market: id_market.clone(),
                oracle_owner: caller,
                block_number_entropy: market_guess.block_number_entropy,
                block_hash_entropy,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_oracle_contract_address(&self) -> AccountId {
            self.env().account_id()
        }

        // #[ink(message)]
        // // pub fn get_entropy_for_market_id(&self, id_market: MarketGuessId) -> (BlockNumber, BlockNumber, ????) {
        // pub fn get_entropy_for_market_id(&self, id_market: MarketGuessId) -> (BlockNumber, BlockNumber) {
        //     let market_guess = match self.market_data.get(id_market) {
        //         Some(data) => data,
        //         None => return Err(Error::NoDataForMarketGuessId),
        //     };
        //     assert!(market_guess.block_hash_entropy != Default::default(), "block hash entropy must be set prior to obtaining entropy");
        //     let block_number_entropy = market_guess.block_number_entropy;
        //     let block_hash_entropy = market_guess.block_hash_entropy;

        //     let entropy = block_hash_entropy.replace(&["0x"][..], "");
            // let str = decode(entropy);
            // let buffer = <[u8; 12]>::from_hex(entropy)?;
            // let str = str::from_utf8(&buffer).expect("invalid buffer length");
            // let sub = &str[..5];
            // ink::env::debug_println!("sub: {}\n", sub);

            // generate random number from block hash
            // don't allow them to use the default value for the random number, they need to have set the
            // block hash entropy to a block hash 

            // (block_number_entropy, block_hash_entropy, ???)
        //     (block_number_entropy, block_hash_entropy)
        // }

        // helper methods
        fn exists_market_data_for_id(&self, id_market: MarketGuessId) -> bool {
            self.market_data.contains(id_market)
        }
    }
}
