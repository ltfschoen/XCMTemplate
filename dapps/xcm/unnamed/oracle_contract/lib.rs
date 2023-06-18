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
    use core::str;
    // use hex;

    // refactor into types file
    pub type MarketGuessId = Vec<u8>;
    pub type OracleOwner = AccountId;

    #[derive(Clone, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct EntropyData(BlockNumber, String, i16, i16);

    /// Emitted when create new market guess for market id.
    #[ink(event)]
    pub struct NewOracleMarketGuessForMarketId {
        #[ink(topic)]
        id_market: String,
        #[ink(topic)]
        oracle_owner: OracleOwner,
        #[ink(topic)]
        block_number_guessed: BlockNumber,
        // Use 3x topics instead of max 4x topics else get error.
        // https://github.com/paritytech/cargo-contract/issues/1160
        // #[ink(topic)]
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
        block_hash_entropy: Option<String>, // Hash
    }

    /// Emitted when set entropy for market id.
    #[ink(event)]
    pub struct SetEntropyForMarketId {
        #[ink(topic)]
        id_market: MarketGuessId,
        oracle_owner: OracleOwner,
        #[ink(topic)]
        block_number_entropy: BlockNumber,
        block_hash_entropy: Option<String>, // Hash
        c1_entropy: i16,
        c2_entropy: i16
    }

    /// Emitted when set block hash entropy for market id.
    #[ink(event)]
    pub struct GeneratedEntropyForMarketId {
        #[ink(topic)]
        id_market: MarketGuessId,
        oracle_owner: OracleOwner,
        #[ink(topic)]
        block_number_entropy: BlockNumber,
        block_hash_entropy: Option<String>, // Hash
        c1_entropy: i16,
        c2_entropy: i16,
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
        block_hash_entropy: Option<String>, // Hash
        /// Market guess period end block number
        block_number_end: BlockNumber,
        /// Entropy random number for coin 1
        c1_entropy: Option<i16>,
        /// Entropy random number for coin 2
        c2_entropy: Option<i16>,
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
        InvalidUTF8Sequence,
        InvalidDigit,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl OracleContract {
        #[ink(constructor)]
        pub fn new(
            id_market: String,
            block_number_guessed: BlockNumber,
            block_number_entropy: BlockNumber,
            block_number_end: BlockNumber,
        ) -> Self {
            ink::env::debug_println!("new");
            let mut instance = Self::default();
            let caller = instance.env().caller();
            assert!(instance.exists_market_data_for_id(id_market.as_bytes()).is_ok(), "market data for given id already exists");
            let block_number_current = Self::env().block_number();
            // TODO - we need to verify that the block hash exists for the block number
            // when they say guessing occurred
            // assert!(
            //     block_number_current > block_number_guessed,
            //     "block number when guessing occurred must be before the current block number"
            // );
            // TODO - 100 and 200 are magic numbers, need something more meaningful
            // assert!(
            //     block_number_entropy - block_number_current > 100,
            //     "block used for entropy must allow sufficient block confirmations after the current \
            //     block and block when guessing occurred for assurance that epoch gets finalized \
            //     incase head susceptible to reorganization for safety"
            // );
            // assert!(
            //     block_number_end - block_number_entropy > 200,
            //     "block when market ends must be allow sufficient block confirmations after the \
            //     block used for entropy"
            // );
            let new_market_guess = MarketGuess {
                id_market: id_market.clone().into_bytes(),
                // must be set to Option<AccountId> to avoid error:
                // the trait `Default` is not implemented for `ink::ink_primitives::AccountId`
                oracle_owner: Some(caller),
                block_number_guessed,
                block_number_entropy,
                block_hash_entropy: None,
                block_number_end,
                c1_entropy: None,
                c2_entropy: None,
            };
            instance.market_data.insert(id_market.clone().into_bytes(), &new_market_guess);
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
        pub fn set_block_for_entropy_for_market_id(
            &mut self,
            id_market: String,
            block_number_entropy: BlockNumber, // always require this even though it may not have changed
            block_hash_entropy: String, // Hash
        ) -> Result<()> {
            let caller: AccountId = self.env().caller();
            ink::env::debug_println!("set_block_for_entropy_for_market_id");
            // assert!(self.exists_market_data_for_id(id_market.as_bytes()).is_ok(), "unable to find market data for given id");
            // TODO - convert Vec<u8> to &str to avoid use of .clone()
            let market_guess = match self.market_data.get(id_market.clone().into_bytes()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            ink::env::debug_println!("block_hash_entropy: {:?}\n", block_hash_entropy);
            ink::env::debug_println!("block_hash_entropy.len(): {:?}\n", block_hash_entropy.len());
            ink::env::debug_println!("market_guess: {:?}\n", market_guess);
            // note: oracle_owner may need to run this function more than once incase entropy block number missed or chain reorg
            // // singleton change of block hash entropy from the value set at instantiation of the contract
            // assert!(
            //     market_guess.block_hash_entropy == None,
            //     "unable to set block hash entropy for market id more than once"
            // );

            let block_number_current = Self::env().block_number();
            // assert!(
            //     block_number_current - market_guess.block_number_entropy > 128,
            //     "unable to update block number entropy for market id again until after \
            //     waiting sufficient blocks after previous so guarantee of waiting until \
            //     validators change after a certain amount of epochs"
            // );
            ink::env::debug_println!("111");
            let block_hash_entropy_no_prefix = block_hash_entropy.replace("0x", "");
            ink::env::debug_println!("222");
            ink::env::debug_println!("block_hash_entropy_no_prefix {:?}", block_hash_entropy_no_prefix);
            assert!(block_hash_entropy_no_prefix.len() == 64, "block hash should be a 256 bit block hash");
            ink::env::debug_println!("333 {:?}, {:?}", market_guess.oracle_owner, caller);
            // FIXME - can't do this or get errors. if called from the main contract then the caller won't be the oracle_owner,
            // instead it'll be the main contract's address
            // if market_guess.oracle_owner != Some(caller) {
            //     return Err(Error::CallerIsNotOracleOwner);
            // }
            ink::env::debug_println!("444");
            let new_market_guess = MarketGuess {
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy_no_prefix.clone()),
                ..market_guess
            };
            ink::env::debug_println!("new_market_guess: {:?}\n", new_market_guess);
            self.market_data.insert(id_market.clone().into_bytes(), &new_market_guess);
            self.env().emit_event(SetBlockHashEntropyForMarketId {
                id_market: id_market.clone().into_bytes(),
                oracle_owner: caller,
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy_no_prefix.clone()),
            });
            Ok(())
        }

        #[ink(message)]
        pub fn set_entropy_for_market_id(
            &mut self,
            id_market: String,
            block_number_entropy: BlockNumber, // always require this even though it may not have changed
            block_hash_entropy: String, // Hash
            c1_entropy: i16,
            c2_entropy: i16,
        ) -> Result<()> {
            ink::env::debug_println!("set_entropy_for_market_id");
            let caller: AccountId = self.env().caller();
            let market_guess = match self.market_data.get(id_market.clone().into_bytes()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            // FIXME - can't do this or get errors.
            // if market_guess.oracle_owner != Some(caller) {
            //     return Err(Error::CallerIsNotOracleOwner);
            // }
            let block_hash_entropy_no_prefix = block_hash_entropy.replace("0x", "");
            assert!(block_hash_entropy_no_prefix.len() == 64, "block hash should be a 256 bit block hash");

            // TODO - replace with `match`
            assert!(
                block_number_entropy == market_guess.block_number_entropy &&
                block_hash_entropy_no_prefix == market_guess.block_hash_entropy.unwrap(),
                "block_number entropy and block hash storage must be set prior to setting entropy for the market"
            );
            assert!(self.exists_market_data_for_id(id_market.as_bytes()).is_ok(), "unable to find market data for given id");
            let new_market_guess = MarketGuess {
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy_no_prefix.clone()),
                c1_entropy: Some(c1_entropy),
                c2_entropy: Some(c2_entropy),
                ..market_guess
            };
            ink::env::debug_println!("new_market_guess: {:?}\n", new_market_guess);
            self.market_data.insert(id_market.clone().into_bytes(), &new_market_guess);
            self.env().emit_event(SetEntropyForMarketId {
                id_market: id_market.into_bytes(),
                oracle_owner: caller,
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy_no_prefix.clone()),
                c1_entropy,
                c2_entropy,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_oracle_contract_address(&self) -> AccountId {
            ink::env::debug_println!("oracle contract address {:?}", self.env().account_id());
            self.env().account_id()
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn get_entropy_for_market_id(&self, id_market: String) -> Result<EntropyData> {  
            let caller: AccountId = self.env().caller();
            let market_guess = match self.market_data.get(id_market.clone().into_bytes()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            ink::env::debug_println!("market_guess.oracle_owner {:?}", market_guess.oracle_owner);
            ink::env::debug_println!("caller {:?}", caller);
            // FIXME - causes `Decode(Error)` since caller account id is smart contract,
            // which differs from the account id of Alice
            // if market_guess.oracle_owner != Some(caller) {
            //     return Err(Error::CallerIsNotOracleOwner)
            // }
            // note: oracle_owner may need to run this function more than once incase entropy block number missed or chain reorg
            // assert!(
            //     market_guess.block_hash_entropy != None,
            //     "block hash entropy must be set prior to obtaining entropy"
            // );
            ink::env::debug_println!("market_guess.block_hash_entropy {:?}",  market_guess.block_hash_entropy);
            let block_number_entropy = market_guess.block_number_entropy;
            // "0xaef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea"
            // note: Hash is [u8; 32] 32 bytes (&[u8]) without 0x prefix and 64 symbols, 32 bytes, 256 bits
            // TODO - replace with `match`
            let block_hash_entropy: String = market_guess.block_hash_entropy.unwrap();
            // let block_hash_entropy: &[u8] =
            //     "aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea".as_bytes();
            // note: changed `block_hash_entropy` to `[u8; 32]` instead of `Hash` so we can get the `.len()`
            assert!(block_hash_entropy.len() == 64, "block hash should be a 256 bit block hash");
            ink::env::debug_println!("block_hash_entropy: {:?}\n", block_hash_entropy);
            // https://peterlyons.com/problog/2017/12/rust-converting-bytes-chars-and-strings/
            let (c1_str, c2_str): (&str, &str) = self.split_last_bytes(block_hash_entropy.as_str());
            ink::env::debug_println!("c1_str: {:?}\n", c1_str);
            ink::env::debug_println!("c2_str: {:?}\n", c2_str);
            let c1_hex = String::from(c1_str);
            let c2_hex = String::from(c2_str);
            ink::env::debug_println!("c1_hex: {:?}", c1_hex);
            ink::env::debug_println!("c2_hex: {:?}", c2_hex);
            // use u16 since max value 65535
            // let without_prefix = hex.trim_start_matches("0x");
            let c1_decimal = match i16::from_str_radix(&c1_hex, 16) {
                Ok(d) => d,
                Err(_e) => return Err(Error::InvalidDigit), 
            };
            let c2_decimal = match i16::from_str_radix(&c2_hex, 16) {
                Ok(d) => d,
                Err(_e) => return Err(Error::InvalidDigit), 
            };
            ink::env::debug_println!("c1_decimal {:?}", c1_decimal);
            ink::env::debug_println!("c2_decimal {:?}", c2_decimal);
            // remainders are 0 or 1 and represent the random side the coin flipped on
            let c1_rem = c1_decimal % 2i16;
            let c2_rem = c2_decimal % 2i16;
            ink::env::debug_println!("c1_rem {:?}", c1_rem);
            ink::env::debug_println!("c2_rem {:?}", c2_rem);

            self.env().emit_event(GeneratedEntropyForMarketId {
                id_market: id_market.clone().into_bytes(),
                oracle_owner: caller,
                block_number_entropy: market_guess.block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy.clone()),
                c1_entropy: c1_rem,
                c2_entropy: c2_rem,
            });

            let entropy_data = EntropyData(block_number_entropy, block_hash_entropy.clone(), c1_rem, c2_rem);
            Ok(entropy_data)
        }

        // get symbols 61-64 for coin1 and 57-60 for coin2 fro the block hash
        fn split_last_bytes<'a>(&'a self, slice: &'a str) -> (&str, &str) {
            let len = String::from(slice).len();
            let sub_slice= &slice[len-8..];
            let (c1, c2) = sub_slice.split_at(4);
            (c1, c2)
        }

        // helper methods
        fn exists_market_data_for_id(&self, id_market: &[u8]) -> Result<bool> {
            let id_market_str = match str::from_utf8(id_market) {
                Ok(v) => v,
                Err(_e) => return Err(Error::InvalidUTF8Sequence),
            };
            Ok(self.market_data.contains(id_market_str.as_bytes().to_vec()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::oracle_contract::OracleContract;
    use std::str::FromStr;

    type BlockNumber = u32;

    /// We test a simple use case of our contract.
    #[ink::test]
    fn it_works() {
        let id_market: String = String::from("my_id");
        let block_number_guessed = 50;
        // override `Self::env().block_number();` for tests
        // See https://substrate.stackexchange.com/questions/8867/how-to-stub-ink-contract-environment-to-produce-fake-values-for-use-in-tests
        let mut new_block_number: BlockNumber = 100;
        ink::env::test::set_block_number::<ink::env::DefaultEnvironment>(new_block_number);
        let block_number_entropy = 228; // must be more than 100 blocks after current block_number
        let block_number_end = 500; // must be more than 200 blocks after block_number_entropy

        let mut oracle_contract = OracleContract::new(
            id_market.clone(),
            block_number_guessed.clone(),
            block_number_entropy.clone(),
            block_number_end.clone(),
        );
        let str_block_hash_entropy: String =
            "aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea".to_string();

        new_block_number = 357; // >128 after block_number_entropy (228 + 128 + 1 = 357)
        ink::env::test::set_block_number::<ink::env::DefaultEnvironment>(new_block_number);
        assert_eq!(oracle_contract.set_block_for_entropy_for_market_id(
            id_market.clone(),
            block_number_entropy.clone(),
            str_block_hash_entropy.clone(),
        ), Ok(()));

        let oracle_contract_address = oracle_contract.get_oracle_contract_address();

        let c1_entropy = 0i16;
        let c2_entropy = 0i16;
        assert_eq!(
            oracle_contract.get_entropy_for_market_id(
                id_market.clone(),
            ).unwrap(),
            (
                block_number_entropy.clone(),
                str_block_hash_entropy.clone(),
                c1_entropy.clone(),
                c2_entropy.clone(),
            )
        );
    }
}