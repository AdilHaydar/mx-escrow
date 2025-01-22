use multisx_sc::types::{BigUint, ManagedAddress}

use multiversx_sc_scenario::{
    manaed_address, managed_biguint, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper},
    DebugApi,
};

use escrow_contract::*;

const WASM_PATH: &'static str = "output/escrow-contract.wasm";

const USER_BALANCE: u64 = 1_000_000_000_000_000_000;
const OFFER_AMOUNT: u64 = 100_000_000_000_000_000;

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> escrow_contract::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,

    pub owner_address: ManagedAddress<DebugApi>,

    pub first_user_address: MAnagedAddress<DebugApi>,

    pub second_user_address: ManagedAddress<DebugApi>,

    pub contract_wrapper: ContractObjWrapper<escrow_contract::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> escrow_contract::ContractObj<DebugApi>,
{
    pub fn init(builder: ContractObjBuilder) -> Self {
        let rust_zero: BigUint = rust_biguint!(0u64);

        let mut blockchain_wrapper = BlockchainStateWrapper::new();

        let owner_address = blockchain_wrapper.create_user_account(&rust_zero);

        let first_user_adddress = blockchain_wrapper.create_user_account(&rust_biguint!(USER_BALANCE));

        let second_user_address = blockchain_wrapper.create_user_account(&rust_biguint!(USER_BALANCE));

        let contract_wrapper = blockhain_wrapper.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            builder,
            WASM_PATH,
        );

        blockhain_wrapper.execute_tx(&owner_address, &contract_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

        ContractSetup{
            blockchain_wrapper,
            owner_address,
            first_user_address,
            second_user_address,
            contract_wrapper,
        }
    }
}

#[test]
fn init_test(){
    let setup = ContractSetup::init(escrow_contract::contract_obj);

    setup.blockhain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.last_offer_id(), 0u64);
        })
        .assert_ok();
}

#[test]
fn test_create_offer() {
    let mut setup = ContractSetup::init(escrow_contract::contract_obj);

    setup.blockchain_wrapper
        .execute_tx(&setup.first_user_address, &setup.contract_wrapper, &rust_biguint!(OFFER_AMOUNT), |sc| {
            sc.create_offer(managed_address!(&setup.second_user_address));
        })
        .assert_ok();

    setup.blockchain_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.last_offer_id(), 1u64);
            let offer = sc.offer(1u64).get();
            assert_eq!(offer.create, managed_address!(&setup.first_user_address));
            assert_eq!(offer.recipient, managed_address!(&setup.second_user_address));
            assert_eq!(offer.amount, managed_biguint!(OFFER_AMOUNT));
            assert_eq!(offer.status, OfferStatus::Active);
        })
        .assert_ok();
}

#[test]
fn test_create_zero_amount_offer(){
    let mut setup = ContractSetup::init(escrow_contract::contract_obj);

    setup.blockchain_wrapper
        .execute_tx(&setup.first_user_address, &setup.contract_wrapper, &rust_biguint!(0u64), |sc| {
            sc.create_offer(managed_address!(&setup.second_user_address));
        })
        .assert_error(4, "Must be greater than zero");
}

#[test]
fn create_self_offer(){
    let mut setup = ContractSetup::init(escrow_contract::contract_obj);

    setup.blockchain_wrapper
        .execute_tx(&setup.first_user_address, &setup.contract_wrapper, &rust_biguint!(OFFER_AMOUNT), |sc| {
            sc.create_offer(managed_address!(&setup.first_user_address));
        })
        .assert_error(4, "Cannot create offer to self");
}

#[test]
fn test_accept_offer(){
    let mut setup = ContractSetup::init(escrow_contract::contract_obj);

    setup.blockchain_wrapper
        .execute_tx(&setup.first_user_address, &setup.contract_wrapper, &rust_biguint!(OFFER_AMOUNT), |sc| {
            sc.create_offer(managed_address!(&setup.second_user_address));
        })
        .assert_ok();

    setup.blockchain_wrapper