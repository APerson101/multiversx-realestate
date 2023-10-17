#![no_std]

multiversx_sc::imports!();
use multiversx_sc::types::heap::Vec;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait RealestateContract {
    #[view]
    #[storage_mapper("quantity")]
    fn quantity(&self) -> SingleValueMapper<BigUint>;
    #[view]
    #[storage_mapper("name")]
    fn name(&self) -> SingleValueMapper<Vec<u8>>;
    #[view]
    #[storage_mapper("address")]
    fn address(&self) -> SingleValueMapper<Vec<u8>>;
    #[view]
    #[storage_mapper("longitude")]
    fn longitude(&self) -> SingleValueMapper<Vec<u8>>;
    #[view]
    #[storage_mapper("latitude")]
    fn latitude(&self) -> SingleValueMapper<Vec<u8>>;
    #[init]
    fn init(
        &self,
        quantity: BigUint,
        name: Vec<u8>,
        address: Vec<u8>,
        longitude: Vec<u8>,
        latitude: Vec<u8>,
    ) {
        self.quantity().set(&quantity);
        self.name().set(&name);
        self.address().set(&address);
        self.latitude().set(&latitude);
        self.longitude().set(&longitude);
    }
}
