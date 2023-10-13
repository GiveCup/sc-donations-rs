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
        let caller = self.blockchain().get_caller();

        let organization_wallet = self.organizations().get(organization_id);
        
        self.send().direct(&organization_wallet, &EgldOrEsdtTokenIdentifier::egld(), 0, &payment);
    }

    #[only_owner]
    #[endpoint(addOrganization)]
    fn add_organization(&self, address: &ManagedAddress) {
        self.organizations().push(&address);
    }

    #[view(getOrganizations)]
    #[storage_mapper("organizations")]
    fn organizations(&self) -> VecMapper<ManagedAddress>;
}
