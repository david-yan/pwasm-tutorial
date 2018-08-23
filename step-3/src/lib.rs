#![no_std]
#![allow(non_snake_case)]
#![feature(use_extern_macros)]
#![feature(proc_macro_gen)]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;

pub mod token {
    use pwasm_ethereum;
    use pwasm_abi::types::*;

    // eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    use pwasm_abi_derive::eth_abi;

    static TOTAL_SUPPLY_KEY: H256 = H256([2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenInterface {
        /// The constructor
        fn constructor(&mut self, _total_supply: U256);
        /// Total amount of tokens
        #[constant]
        fn totalSupply(&mut self) -> U256;
    }

    pub struct TokenContract;

    impl TokenInterface for TokenContract {
        fn constructor(&mut self, total_supply: U256) {
            // Set up the total supply for the token
            pwasm_ethereum::write(&TOTAL_SUPPLY_KEY, &total_supply.into());
        }

        fn totalSupply(&mut self) -> U256 {
            U256::from_big_endian(&pwasm_ethereum::read(&TOTAL_SUPPLY_KEY))
        }
    }
}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}
