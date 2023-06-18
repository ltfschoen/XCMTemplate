#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unnamed {
    use ink::env::{
        call::{
            build_create,
            build_call,
            ExecutionInput,
            Selector,
        },
        DefaultEnvironment,
    };

    use oracle_contract::OracleContractRef;

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    // refactor into types file
    pub type OracleOwner = AccountId;

    #[derive(Clone, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct EntropyData(BlockNumber, String, i16, i16);

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default)]
    #[ink(storage)]
    pub struct Unnamed {
        /// Store a reference to the `OracleContract`.
        oracle_contract: Option<OracleContractRef>,
        oracle_contract_address: Option<AccountId>,
        owner: Option<OracleOwner>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not oracle contract address exists.
        NoOracleContractAddress,
        ResponseError,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Unnamed {
        /// Constructor that instantiates the OracleContract using its uploaded `code_hash`
        #[ink(constructor)]
        pub fn new(
            oracle_contract_code_hash: Hash,
            oracle_contract_address: AccountId,
            id_market: String,
            block_number_guessed: BlockNumber,
            block_number_entropy: BlockNumber,
            block_number_end: BlockNumber,
        ) -> Self {
            let instance = Self::default();
            let caller = instance.env().caller();
            let oracle_contract = build_create::<OracleContractRef>()
                .code_hash(oracle_contract_code_hash)
                .gas_limit(100000000000)
                .endowment(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new")))
                        .push_arg(id_market)
                        .push_arg(block_number_guessed)
                        .push_arg(block_number_entropy)
                        .push_arg(block_number_end)
                )
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .returns::<OracleContractRef>()
                .instantiate();

            Self {
                oracle_contract: Some(oracle_contract),
                oracle_contract_address: Some(oracle_contract_address),
                owner: Some(caller),
            }
        }

        /// Using the `OracleContractRef` we can call all the messages of the `OracleContract`
        #[ink(message)]
        pub fn set_block_for_entropy_for_market_id(
            &mut self,
            id_market: String,
            block_number_entropy: BlockNumber, // always require this even though it may not have changed
            block_hash_entropy: String, // Hash
        ) -> Result<()> {
            ink::env::debug_println!("&self.oracle_contract_address {:?}", &self.oracle_contract_address);
            match &self.oracle_contract_address {
                Some(c) => {
                    // using CallBuilder
                    // https://use.ink/basics/cross-contract-calling#callbuilder
                    build_call::<DefaultEnvironment>()
                        .call(c.clone())
                        .gas_limit(100000000000)
                        .transferred_value(0)
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("set_block_for_entropy_for_market_id")))
                                .push_arg(id_market)
                                .push_arg(block_number_entropy)
                                .push_arg(block_hash_entropy.clone())
                        )
                        .returns::<()>()
                        .invoke();
                    Ok(())
                },
                None => return Err(Error::NoOracleContractAddress),
            }
        }

        /// Using the `OracleContractRef` we can call all the messages of the `OracleContract`
        #[ink(message)]
        pub fn set_entropy_for_market_id(
            &mut self,
            id_market: String,
            block_number_entropy: BlockNumber, // always require this even though it may not have changed
            block_hash_entropy: String, // Hash
            c1_entropy: i16,
            c2_entropy: i16,
        ) -> Result<()> {
            ink::env::debug_println!("&self.oracle_contract_address {:?}", &self.oracle_contract_address);
            match &self.oracle_contract_address {
                Some(c) => {
                    // using CallBuilder
                    // https://use.ink/basics/cross-contract-calling#callbuilder
                    build_call::<DefaultEnvironment>()
                        .call(c.clone())
                        .gas_limit(100000000000)
                        .transferred_value(0)
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("set_entropy_for_market_id")))
                                .push_arg(id_market)
                                .push_arg(block_number_entropy)
                                .push_arg(block_hash_entropy.clone())
                                .push_arg(c1_entropy)
                                .push_arg(c2_entropy)
                        )
                        .returns::<()>()
                        .invoke();
                    Ok(())
                },
                None => return Err(Error::NoOracleContractAddress),
            }
        }

        /// Using the `OracleContractRef` we can call all the messages of the `OracleContract`
        #[ink(message)]
        pub fn get_entropy_for_market_id(&self, id_market: String) -> Result<EntropyData> {
            match &self.oracle_contract_address {
                Some(c) => {
                    let res =
                        build_call::<DefaultEnvironment>()
                            .call(c.clone())
                            .gas_limit(100000000000)
                            .exec_input(
                                ExecutionInput::new(Selector::new(ink::selector_bytes!("get_entropy_for_market_id")))
                                    .push_arg(id_market)
                            )
                            .returns::<EntropyData>()
                            .try_invoke()
                            .expect("Error calling get_entropy_for_market_id.");
                    match res {
                        Ok(tuple) => {
                            ink::env::debug_println!("tuple {:?}", tuple);
                            return Ok(tuple);
                        },
                        Err(e) => {
                            ink::env::debug_println!("error {:?}", e);
                            return Err(Error::ResponseError);
                        },
                    };
                },
                None => return Err(Error::NoOracleContractAddress),
            }
        }

        /// Using the `OracleContractRef` we can call all the messages of the `OracleContract`
        #[ink(message)]
        pub fn get_oracle_contract_address(&self) -> Result<AccountId> {
            match &self.oracle_contract_address {
                Some(c) => {
                    let res =
                        build_call::<DefaultEnvironment>()
                            .call(c.clone())
                            .gas_limit(100000000000)
                            .exec_input(
                                ExecutionInput::new(Selector::new(ink::selector_bytes!("get_oracle_contract_address")))
                            )
                            .returns::<AccountId>()
                            .try_invoke()
                            .expect("Error calling get_oracle_contract_address.");
                    match res {
                        Ok(contract_address) => {
                            ink::env::debug_println!("contract_address {:?}", contract_address);
                            return Ok(contract_address);
                        },
                        Err(e) => {
                            ink::env::debug_println!("error {:?}", e);
                            return Err(Error::ResponseError);
                        },
                    };
                },
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
