#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint]
    fn donate(&self, org_id: usize) {
        let org_address = self.orgs().get(&org_id).unwrap_or_else(|| sc_panic!("Organization not found."));
        let payment = self.call_value().egld_or_single_esdt();
        
        self.increase_balance(&org_address, &payment.token_identifier, payment.token_nonce, &payment.amount);
    }
}
