#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod basic_contract_caller {
    use ink::env::{
        call::{
            build_create,
            build_call,
            ExecutionInput,
            Selector,
        },
        DefaultEnvironment,
    };
    
    /// We import the generated `ContractRef` of our other contract.
    ///
    /// Note that the other contract must have re-exported it (`pub use
    /// OtherContractRef`) for us to have access to it.
    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct BasicContractCaller {
        /// We specify that our contract will store a reference to the `OtherContract`.
        other_contract: Option<OtherContractRef>,
        other_contract_address: Option<AccountId>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not other contract address exists.
        NoOtherContractAddress,
        ResponseError,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl BasicContractCaller {
        /// In order to use the `OtherContract` we first need to **instantiate** it.
        ///
        /// To do this we will use the uploaded `code_hash` of `OtherContract`.
        #[ink(constructor)]
        pub fn new(other_contract_code_hash: Hash, other_contract_address: AccountId) -> Self {
            // using `CreateBuilder` to instantiate contract
            // https://use.ink/basics/cross-contract-calling#createbuilder
            let other_contract: OtherContractRef = build_create::<OtherContractRef>()
                .code_hash(other_contract_code_hash)
                // https://substrate.stackexchange.com/questions/3992/i-get-a-the-executed-contract-exhausted-its-gas-limit-when-attempting-to-inst
                .gas_limit(100000000000)
                // https://substrate.stackexchange.com/questions/8445/cross-contract-instantiation-failed-with-transferfailed/8447#8447
                .endowment(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("new")))
                        .push_arg(true)
                )
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .returns::<OtherContractRef>()
                .instantiate();

            Self {
                other_contract: Some(other_contract),
                other_contract_address: Some(other_contract_address),
            }
        }

        #[ink(message)]
        pub fn get(&mut self) -> Result<bool> {
            match &self.other_contract_address {
                Some(c) => {
                    let res = build_call::<DefaultEnvironment>()
                        .call(c.clone())
                        .gas_limit(100000000000)
                        // .transferred_value(10) // TransferFailed
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("get")))
                        )
                        .returns::<bool>()
                        // https://use.ink/basics/cross-contract-calling#builder-error-handling
                        .try_invoke()
                        .expect("Error calling get.");
                    match res {
                        Ok(is_flipped) => {
                            ink::env::debug_println!("is_flipped {:?}", is_flipped);
                            return Ok(is_flipped);
                        },
                        Err(e) => {
                            ink::env::debug_println!("error {:?}", e);
                            return Err(Error::ResponseError);
                        },
                    };
                },
                None => return Err(Error::NoOtherContractAddress),
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) -> Result<()> {
            match &self.other_contract_address {
                Some(c) => {
                    // using CallBuilder
                    // https://use.ink/basics/cross-contract-calling#callbuilder
                    let _ = build_call::<DefaultEnvironment>()
                        .call(c.clone())
                        .gas_limit(100000000000)
                        .transferred_value(0) // TransferFailed if non-zero
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("flip")))
                        )
                        .returns::<()>()
                        .try_invoke()
                        .expect("Error calling flip.");
                    Ok(())
                },
                None => return Err(Error::NoOtherContractAddress),
            }
        }

        #[ink(message)]
        pub fn flip_and_get(&mut self) -> Result<bool> {
            match &self.other_contract_address {
                Some(c) => {
                    let _ = build_call::<DefaultEnvironment>()
                        .call(c.clone())
                        .gas_limit(100000000000)
                        .transferred_value(0) // TransferFailed if non-zero
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("flip")))
                        )
                        .returns::<()>()
                        .try_invoke()
                        .expect("Error calling flip.");
                    let res = build_call::<DefaultEnvironment>()
                        .call(c.clone())
                        .gas_limit(100000000000)
                        .exec_input(
                            ExecutionInput::new(Selector::new(ink::selector_bytes!("get")))
                        )
                        .returns::<bool>()
                        .try_invoke()
                        .expect("Error calling get.");
                    match res {
                        Ok(is_flipped) => {
                            ink::env::debug_println!("is_flipped {:?}", is_flipped);
                            return Ok(is_flipped);
                        },
                        Err(e) => {
                            ink::env::debug_println!("error {:?}", e);
                            return Err(Error::ResponseError);
                        },
                    };
                },
                None => return Err(Error::NoOtherContractAddress),
            }
        }

        #[ink(message)]
        pub fn get_other_contract_address(&self) -> Result<AccountId> {
            match &self.other_contract_address {
                Some(c) => {
                    let res =
                        build_call::<DefaultEnvironment>()
                            .call(c.clone())
                            .gas_limit(100000000000)
                            // .transferred_value(10) // TransferFailed
                            .exec_input(
                                ExecutionInput::new(Selector::new(ink::selector_bytes!("get_other_contract_address")))
                            )
                            .returns::<AccountId>()
                            .try_invoke()
                            .expect("Error calling get_other_contract_address.");
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
                None => return Err(Error::NoOtherContractAddress),
            }
        }
    }
}
