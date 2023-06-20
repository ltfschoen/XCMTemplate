#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp22 {
    
    // imports from openbrush
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Ipsp22 {
        #[storage_field]
        psp22: psp22::Data,
    }
    
    // Section contains default implementation without any modifications
    impl PSP22 for Ipsp22 {}
     
    impl Ipsp22 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
            _instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
            _instance
        }
    }
}
