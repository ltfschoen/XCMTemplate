#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unnamed {
    use oracle_contract::OracleContractRef;

    use ink::prelude::vec::Vec;

    // refactor into types file
    pub type MarketGuessId = Vec<u8>;
    pub type OracleOwner = AccountId;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default)]
    #[ink(storage)]
    pub struct Unnamed {
        /// Store a reference to the `OracleContract`.
        oracle_contract: Option<OracleContractRef>,
        owner: Option<OracleOwner>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not oracle contract address exists.
        NoOracleContractAddress,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Unnamed {
        /// Constructor that instantiates the OracleContract using its uploaded `code_hash`
        #[ink(constructor)]
        pub fn new(
            oracle_contract_code_hash: Hash,
            id_market: MarketGuessId,
            block_number_guessed: BlockNumber,
            block_number_entropy: BlockNumber,
            block_number_end: BlockNumber,
        ) -> Self {
            let instance = Self::default();
            let caller = instance.env().caller();
            let oracle_contract = OracleContractRef::new(
                    id_market,
                    block_number_guessed,
                    block_number_entropy,
                    block_number_end,
                )
                .code_hash(oracle_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self {
                oracle_contract: Some(oracle_contract),
                owner: Some(caller),
            }
        }

        /// Using the `OracleContractRef` we can call all the messages of the `OracleContract`
        #[ink(message)]
        pub fn get_oracle_contract_address(&self) -> Result<AccountId> {
            match &self.oracle_contract {
                Some(c) => Ok(c.get_oracle_contract_address()),
                None => return Err(Error::NoOracleContractAddress),
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let unnamed = Unnamed::default();
        //     assert_eq!(unnamed.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut unnamed = Unnamed::new(false);
        //     assert_eq!(unnamed.get(), false);
        //     unnamed.flip();
        //     assert_eq!(unnamed.get(), true);
        // }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // // Given
            // let constructor = UnnamedRef::default();

            // // When
            // let contract_account_id = client
            //     .instantiate("unnamed", &ink_e2e::alice(), constructor, 0, None)
            //     .await
            //     .expect("instantiate failed")
            //     .account_id;

            // // Then
            // let get = build_message::<UnnamedRef>(contract_account_id.clone())
            //     .call(|unnamed| unnamed.get());
            // let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            // assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // // Given
            // let constructor = UnnamedRef::new(false);
            // let contract_account_id = client
            //     .instantiate("unnamed", &ink_e2e::bob(), constructor, 0, None)
            //     .await
            //     .expect("instantiate failed")
            //     .account_id;

            // let get = build_message::<UnnamedRef>(contract_account_id.clone())
            //     .call(|unnamed| unnamed.get());
            // let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            // assert!(matches!(get_result.return_value(), false));

            // // When
            // let flip = build_message::<UnnamedRef>(contract_account_id.clone())
            //     .call(|unnamed| unnamed.flip());
            // let _flip_result = client
            //     .call(&ink_e2e::bob(), flip, 0, None)
            //     .await
            //     .expect("flip failed");

            // // Then
            // let get = build_message::<UnnamedRef>(contract_account_id.clone())
            //     .call(|unnamed| unnamed.get());
            // let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            // assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
