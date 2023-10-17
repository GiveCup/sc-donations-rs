#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}


    #[payable("*")]
    #[endpoint]
    fn donate(&self, organization_id: usize) {
        let payment = self.call_value().egld_value();
        require(payment > 0, "Payment must be greater than 0");

        let caller = self.blockchain().get_caller();

        let organization_wallet = self.organizations().get(organization_id);
        require(organization_wallet.is_some(), "Organization does not exist");

        let organization_balance = self.organization_balance(&organization_wallet);
        self.organization_balance(&organization_wallet).set(&organization_balance + &payment);
    }

    #[only_owner]
    #[endpoint(addOrganization)]
    fn add_organization(&self, address: &ManagedAddress) {
        self.organizations().push(&address);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        
    }

    #[view(getOrganizations)]
    #[storage_mapper("organizations")]
    fn organizations(&self) -> VecMapper<&ManagedAddress>;

    #[storage_mapper("organizationBalance")]
    fn organization_balance(&self, address: &ManagedAddress) -> SingleValueMapper<&BigUint>;
}
