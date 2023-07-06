#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Re-export the `ContractRef` generated by the ink! codegen.
///
/// This let's other crates which pull this contract in as a dependency to interact
/// with this contract in a type-safe way.
pub use self::other_contract::OtherContractRef;

#[ink::contract]
mod other_contract {

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        UnknownError,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct OtherContract {
        value: bool,
    }

    impl OtherContract {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn flip(&mut self) -> Result<bool> {
            ink::env::debug_println!(
                "received payment with flip: {}",
                self.env().transferred_value()
            );

            self.value = !self.value;
            return Ok(self.value)
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_other_contract_address(&self) -> AccountId {
            ink::env::debug_println!("oracle contract address {:?}", self.env().account_id());
            self.env().account_id()
        }
    }
}
