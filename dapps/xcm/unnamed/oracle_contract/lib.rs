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
        block_hash_entropy: Option<[u8; 32]>, // Hash
    }

    /// Emitted when set entropy for market id.
    #[ink(event)]
    pub struct SetEntropyForMarketId {
        #[ink(topic)]
        id_market: MarketGuessId,
        oracle_owner: OracleOwner,
        #[ink(topic)]
        block_number_entropy: BlockNumber,
        block_hash_entropy: Option<[u8; 32]>, // Hash
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
        block_hash_entropy: Option<[u8; 32]>, // Hash
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
        block_hash_entropy: Option<[u8; 32]>, // Hash
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
            assert!(!instance.exists_market_data_for_id(id_market.clone()), "market data for given id already exists");
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
                block_hash_entropy: None,
                block_number_end,
                c1_entropy: None,
                c2_entropy: None,
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
        pub fn set_block_for_entropy_for_market_id(
            &mut self,
            id_market: MarketGuessId,
            block_number_entropy: BlockNumber, // always require this even though it may not have changed
            block_hash_entropy: [u8; 32], // Hash
        ) -> Result<()> {
            let caller: AccountId = self.env().caller();
            // assert!(self.exists_market_data_for_id(id_market), "unable to find market data for given id");
            // TODO - convert Vec<u8> to &str to avoid use of .clone()
            let market_guess = match self.market_data.get(id_market.clone()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            // note: oracle_owner may need to run this function more than once incase entropy block number missed or chain reorg
            // // singleton change of block hash entropy from the value set at instantiation of the contract
            // assert!(
            //     market_guess.block_hash_entropy == None,
            //     "unable to set block hash entropy for market id more than once"
            // );

            assert!(
                block_number_entropy - market_guess.block_number_entropy > 128,
                "unable to update block number entropy for market id again until after \
                waiting sufficient blocks after previous so guarantee of waiting until \
                validators change after a certain amount of epochs"
            );

            if market_guess.oracle_owner != Some(caller) {
                return Err(Error::CallerIsNotOracleOwner)
            }
            let new_market_guess = MarketGuess {
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy),
                ..market_guess
            };
            self.market_data.insert(id_market.clone(), &new_market_guess);
            self.env().emit_event(SetBlockHashEntropyForMarketId {
                id_market: id_market.clone(),
                oracle_owner: caller,
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy),
            });
            Ok(())
        }

        #[ink(message)]
        pub fn set_entropy_for_market_id(
            &mut self,
            id_market: MarketGuessId,
            block_number_entropy: BlockNumber, // always require this even though it may not have changed
            block_hash_entropy: [u8; 32], // Hash
            c1_entropy: i16,
            c2_entropy: i16,
        ) -> Result<()> {
            let caller: AccountId = self.env().caller();
            let market_guess = match self.market_data.get(id_market.clone()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            if market_guess.oracle_owner != Some(caller) {
                return Err(Error::CallerIsNotOracleOwner)
            }
            assert!(
                block_number_entropy == market_guess.block_number_entropy &&
                block_hash_entropy == market_guess.block_hash_entropy.unwrap(),
                "block_number entropy and block hash storage must be set prior to setting entropy for the market"
            );
            assert!(self.exists_market_data_for_id(id_market.clone()), "unable to find market data for given id");
            let new_market_guess = MarketGuess {
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy),
                c1_entropy: Some(c1_entropy),
                c2_entropy: Some(c2_entropy),
                ..market_guess
            };
            self.market_data.insert(id_market.clone(), &new_market_guess);
            self.env().emit_event(SetEntropyForMarketId {
                id_market: id_market.clone(),
                oracle_owner: caller,
                block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy),
                c1_entropy,
                c2_entropy,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_oracle_contract_address(&self) -> AccountId {
            self.env().account_id()
        }

        #[ink(message)]
        pub fn get_entropy_for_market_id(&self, id_market: MarketGuessId) -> Result<(BlockNumber, [u8; 32], i16, i16)> {
            let caller: AccountId = self.env().caller();
            let market_guess = match self.market_data.get(id_market.clone()) {
                Some(data) => data,
                None => return Err(Error::NoDataForMarketGuessId),
            };
            if market_guess.oracle_owner != Some(caller) {
                return Err(Error::CallerIsNotOracleOwner)
            }
            // note: oracle_owner may need to run this function more than once incase entropy block number missed or chain reorg
            // assert!(
            //     market_guess.block_hash_entropy != None,
            //     "block hash entropy must be set prior to obtaining entropy"
            // );
            let block_number_entropy = market_guess.block_number_entropy;
            // e."0xaef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea"
            // note: Hash is [u8; 32] 32 bytes (&[u8]) without 0x prefix and 64 symbols, 32 bytes, 256 bits
            let block_hash_entropy: [u8; 32] = market_guess.block_hash_entropy.unwrap();
            // let block_hash_entropy: &[u8] =
            //     "aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea".as_bytes();
            // note: changed `block_hash_entropy` to `[u8; 32]` instead of `Hash` so we can get the `.len()`
            assert!(block_hash_entropy.len() == 64, "block hash should be a 256 bit block hash");
            ink::env::debug_println!("block_hash_entropy: {:?}\n", block_hash_entropy);
            // https://peterlyons.com/problog/2017/12/rust-converting-bytes-chars-and-strings/
            let (c1_u8a, c2_u8a): (&[u8], &[u8]) = self.last_bytes(&block_hash_entropy);
            ink::env::debug_println!("c1_u8a: {:?}\n", c1_u8a);
            ink::env::debug_println!("c2_u8a: {:?}\n", c2_u8a);
            let c1_hex = String::from_utf8_lossy(&c1_u8a);
            let c2_hex = String::from_utf8_lossy(&c2_u8a);
            ink::env::debug_println!("c1_hex: {:?}", c1_hex);
            ink::env::debug_println!("c2_hex: {:?}", c2_hex);
            // use u16 since max value 65535
            // let without_prefix = hex.trim_start_matches("0x");
            let c1_decimal = i16::from_str_radix(&c1_hex, 16).unwrap();
            let c2_decimal = i16::from_str_radix(&c2_hex, 16).unwrap();
            ink::env::debug_println!("c1_decimal {:?}", c1_decimal);
            ink::env::debug_println!("c2_decimal {:?}", c2_decimal);
            // remainders are 0 or 1 and represent the random side the coin flipped on
            let c1_rem = c1_decimal % 2i16;
            let c2_rem = c2_decimal % 2i16;
            ink::env::debug_println!("c1_rem {:?}", c1_rem);
            ink::env::debug_println!("c2_rem {:?}", c2_rem);

            self.env().emit_event(GeneratedEntropyForMarketId {
                id_market: id_market.clone(),
                oracle_owner: caller,
                block_number_entropy: market_guess.block_number_entropy,
                block_hash_entropy: Some(block_hash_entropy),
                c1_entropy: c1_rem,
                c2_entropy: c2_rem,
            });

            Ok((block_number_entropy, block_hash_entropy, c1_rem, c2_rem))
        }

        // get symbols 61-64 for coin1 and 57-60 for coin2 fro the block hash
        fn last_bytes<'a>(&'a self, slice: &'a [u8; 32]) -> (&[u8], &[u8]) {
            let bytes = slice.split_at(slice.len() - 8).1;
            (bytes.split_at(bytes.len() - 4).0, bytes.split_at(bytes.len() - 4).1)
        }

        // helper methods
        fn exists_market_data_for_id(&self, id_market: MarketGuessId) -> bool {
            self.market_data.contains(id_market)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::oracle_contract::OracleContract;
    // use ethers::core::types::H256;
    use ethers_core::types::{H256};
    use std::str::FromStr;
    // use ink::primitives::{Hash};

    use ink::env::{
        Environment,
        DefaultEnvironment,
    };

    #[ink::chain_extension]
    pub trait CustomBlocknumberExtension {
        type ErrorCode = CustomBlocknumberError;

        #[ink(extension = 0x1111)]
        fn fake_current_block_number() -> Result<BlockNumber> {
            // TODO - figure out how to stub the ink! environment functions
            // See https://substrate.stackexchange.com/questions/8867/how-to-stub-ink-contract-environment-to-produce-fake-values-for-use-in-tests
        };
    }    

    /// An environment using default ink environment types
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CustomEnvironment {}

    impl Environment for CustomEnvironment {
        const MAX_EVENT_TOPICS: usize =
            <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

        type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
        type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
        type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
        type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;
        type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;

        type ChainExtension = crate::CustomBlocknumberExtension;
    }

    /// We test a simple use case of our contract.
    #[ink::test(env = crate::CustomEnvironment)]
    fn it_works() {
        assert!(self.env().extension().fake_current_block_number(), 200);
        let id_market: Vec<u8> = String::from_utf8("my_id".into()).unwrap().into();
        let block_number_guessed = 100;
        let block_number_entropy = 228;
        let block_number_end = 300;
        let mut oracle_contract = OracleContract::new(
            id_market.clone(),
            block_number_guessed.clone(),
            block_number_entropy.clone(),
            block_number_end.clone(),
        );
        let str_block_hash_entropy: String =
            "0xaef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea".to_string();

        let hash_block_hash_entropy: H256 = str_block_hash_entropy.as_str().parse::<H256>().unwrap();
        let bytes_block_hash_entropy: [u8; 32] = hash_block_hash_entropy.to_fixed_bytes();
        assert_eq!(oracle_contract.set_block_for_entropy_for_market_id(
            id_market.clone(),
            block_number_entropy.clone(),
            bytes_block_hash_entropy.clone(),
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
                bytes_block_hash_entropy.clone(),
                c1_entropy.clone(),
                c2_entropy.clone(),
            )
        );
    }
}