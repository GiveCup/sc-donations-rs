// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    contract
    (
        add_project
        add_matching_funds
        contribute
        calculate_match
        distribute_funds
        calculateRawMatch
        setMatchingFund
        setTotalRawMatch
        setProjectRawMatch
        setProjectContributions
        getMatchingFund
        getProjectRawMatch
        getTotalRawMatch
        getRawMatch
        getProjectContributions
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
