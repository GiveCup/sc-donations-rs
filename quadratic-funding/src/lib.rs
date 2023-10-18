#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use core::ops::Deref;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct Contribution<M: ManagedTypeApi> {
    contributor: ManagedAddress<M>,
    amount: BigUint<M>,
}

#[multiversx_sc::contract]
pub trait QuadraticFundingContract {

    #[init]
    fn init(&self) {
    }

    #[endpoint]
    fn add_project(&self, project_address: ManagedAddress) {
        self.projects().push(&project_address);
    }

    #[payable("*")]
    #[endpoint]
    fn add_matching_funds(&self) {
        let amount = self.call_value().egld_value();
        let matching_fund = self.matching_fund().get();

        // self.matching_fund().set(&matching_fund + &amount);
    }

    // Allow users to contribute to a project.
    #[payable("*")]
    #[endpoint]
    fn contribute(&self, project_id: usize) {
        let payment = self.call_value().egld_value();

        require!(payment.deref() > &BigUint::from(0u64), "Payment must be greater than 0");

        let caller = self.blockchain().get_caller();

        // TODO: handle the case when the project does not exist
        let project_wallet = self.projects().get(project_id);
        let contributions = self.project_contributions(project_id);


        // Logic to track each user's contributions separately.
    }

    // Calculate the matched fund for a project.
    #[view]
    fn calculate_match(&self, project_id: ManagedBuffer) -> BigUint {
        // Logic to calculate how much matching fund a project will receive.
        BigUint::zero() // Placeholder return.
    }

    // Distribute the funds among all projects.
    // This can be called after the funding round ends.
    #[endpoint]
    fn distribute_funds(&self) {
        // Logic to distribute the matching funds to the projects.
    }

    fn calculate_raw_match(&self, project_id: usize) -> BigUint {
        let project_contributions = self.project_contributions(project_id);
        let mut total_contributions = BigUint::zero();

        for contribution in project_contributions.iter() {
            total_contributions += contribution.amount;
        }

        let matching_fund = self.matching_fund().get();
        let total_raw_match = self.total_raw_match().get();

        let raw_match = (total_contributions * matching_fund) / total_raw_match;

        raw_match
    }

    // storage modifiers
    #[only_owner]
    #[endpoint(setMatchingFund)]
    fn set_matching_fund(&self, matching_fund: BigUint) {
        self.matching_fund().set(&matching_fund);
    }

    #[only_owner]
    #[endpoint(setTotalRawMatch)]
    fn set_total_raw_match(&self, total_raw_match: BigUint) {
        self.total_raw_match().set(&total_raw_match);
    }

    #[only_owner]
    #[endpoint(setProjectRawMatch)]
    fn set_project_raw_match(&self, project_address: ManagedAddress, raw_match: BigUint) {
        self.projects_raw_match().insert(project_address, raw_match);
    }

    // #[only_owner]
    // #[endpoint(setProjectContributions)]
    // fn set_project_contributions(&self, project_id: usize, contributions: Vec<Contribution<Self::Api>>) {
    //     self.project_contributions(project_id).push(&contributions);
    // }

    // Structures and mappers for storage
    #[storage_mapper("projects")]
    fn projects(&self) -> VecMapper<ManagedAddress>;

    #[storage_mapper("matching_fund")]
    fn matching_fund(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("projects_raw_match")]
    fn projects_raw_match(&self) -> MapMapper<ManagedAddress,BigUint>;

    #[storage_mapper("total_row_match")]
    fn total_raw_match(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("project_contributions")]
    fn project_contributions(&self, project_id: usize) -> VecMapper<Self::Api, Contribution<Self::Api>>;
}

