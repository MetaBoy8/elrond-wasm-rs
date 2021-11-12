use std::rc::Rc;

use elrond_wasm::{
    api::VMApi,
    contract_base::ContractBase,
    sc_error,
    types::{Address, BigUint, ManagedFrom, SCResult, H256},
};
use elrond_wasm_debug::{
    tx_mock::{TxCache, TxInput},
    world_mock::{AccountData, AccountEsdt},
    BlockchainMock, DebugApi, HashMap,
};
use rust_testing_framework_tester::*;

#[test]
fn test_call_sum_biguint() {
    let sc = rust_testing_framework_tester::contract_obj(DebugApi::dummy());
    let api = sc.raw_vm_api(); // will be removed entirely in the next version

    let first = BigUint::managed_from(api.clone(), 2u64);
    let second = BigUint::managed_from(api.clone(), 3u64);
    let expected_result = first.clone() + second.clone();
    let actual_result = sc.sum(first, second);
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_call_sum_sc_result_ok() {
    let sc = rust_testing_framework_tester::contract_obj(DebugApi::dummy());
    let api = sc.raw_vm_api(); // will be removed entirely in the next version

    let first = BigUint::managed_from(api.clone(), 2u64);
    let second = BigUint::managed_from(api.clone(), 3u64);
    let expected_result = SCResult::Ok(first.clone() + second.clone());
    let actual_result = sc.sum_sc_result(first, second);
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_call_sum_sc_result_err() {
    let sc = rust_testing_framework_tester::contract_obj(DebugApi::dummy());
    let api = sc.raw_vm_api(); // will be removed entirely in the next version

    let first = BigUint::managed_from(api.clone(), 0u64);
    let second = BigUint::managed_from(api.clone(), 3u64);
    let expected_result: SCResult<BigUint<DebugApi>> = sc_error!("Non-zero required");
    let actual_result = sc.sum_sc_result(first, second);
    assert_eq!(expected_result, actual_result);
}

#[test]
fn test_sc_set_tx_input() {
    let mut blockchain_mock = BlockchainMock::new();
    let caller_addr = Address::from([1u8; 32]);

    let mut sc_addr_raw = [1u8; 32];
    for i in 0..8 {
        sc_addr_raw[i] = 0;
    }
    let sc_addr = Address::from(sc_addr_raw);

    // add the address to the state, with 1000 EGLD balance
    blockchain_mock.add_account(AccountData {
        address: caller_addr.clone(),
        nonce: 0,
        egld_balance: num_bigint::BigUint::from(1_000u32),
        esdt: AccountEsdt::default(),
        storage: HashMap::new(),
        username: Vec::new(),
        contract_path: None,
        contract_owner: None,
    });

    // add sc to the state, with 2000 EGLD balance
    blockchain_mock.add_account(AccountData {
        address: sc_addr.clone(),
        nonce: 0,
        egld_balance: num_bigint::BigUint::from(2_000u32),
        esdt: AccountEsdt::default(),
        storage: HashMap::new(),
        username: Vec::new(),
        contract_path: None,
        contract_owner: None,
    });

    let tx_input = TxInput {
        from: caller_addr.clone(),
        to: sc_addr.clone(),
        egld_value: num_bigint::BigUint::from(0u32),
        esdt_values: Vec::new(),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: u64::MAX,
        gas_price: 0,
        tx_hash: H256::zero(),
    };

    let rc_world = Rc::new(blockchain_mock);
    let debug_api = DebugApi::new(tx_input, TxCache::new(rc_world));
    let sc = rust_testing_framework_tester::contract_obj(debug_api);
    let api = sc.raw_vm_api();

    let expected_balance = BigUint::managed_from(api.clone(), 2_000u32);
    let actual_balance = sc.get_egld_balance();
    assert_eq!(expected_balance, actual_balance);

    let actual_caller = sc.get_caller_legacy();
    assert_eq!(caller_addr, actual_caller);
}

#[test]
fn test_sc_payment() {
    let mut blockchain_mock = BlockchainMock::new();
    let caller_addr = Address::from([1u8; 32]);

    let mut sc_addr_raw = [1u8; 32];
    for i in 0..8 {
        sc_addr_raw[i] = 0;
    }
    let sc_addr = Address::from(sc_addr_raw);

    // add the address to the state, with 1000 EGLD balance
    blockchain_mock.add_account(AccountData {
        address: caller_addr.clone(),
        nonce: 0,
        egld_balance: num_bigint::BigUint::from(1_000u32),
        esdt: AccountEsdt::default(),
        storage: HashMap::new(),
        username: Vec::new(),
        contract_path: None,
        contract_owner: None,
    });

    // add sc to the state, with 2000 EGLD balance
    blockchain_mock.add_account(AccountData {
        address: sc_addr.clone(),
        nonce: 0,
        egld_balance: num_bigint::BigUint::from(2_000u32),
        esdt: AccountEsdt::default(),
        storage: HashMap::new(),
        username: Vec::new(),
        contract_path: None,
        contract_owner: None,
    });

    let tx_input = TxInput {
        from: caller_addr.clone(),
        to: sc_addr.clone(),
        egld_value: num_bigint::BigUint::from(1_000u32),
        esdt_values: Vec::new(),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: u64::MAX,
        gas_price: 0,
        tx_hash: H256::zero(),
    };

    let rc_world = Rc::new(blockchain_mock);
    let debug_api = DebugApi::new(tx_input, TxCache::new(rc_world.clone()));
    let sc = rust_testing_framework_tester::contract_obj(debug_api);
    {
        let actual_payment_amount = sc.receive_egld();
        let expected_payment_amount = BigUint::managed_from(sc.raw_vm_api(), 1_000u32);
        assert_eq!(actual_payment_amount, expected_payment_amount);
    }

    let api = into_api(sc);
    let bu = api.into_blockchain_updates();

    blockchain_mock = Rc::try_unwrap(rc_world).unwrap();
    bu.apply(&mut blockchain_mock);

    let user_acc_after = blockchain_mock.accounts.get(&caller_addr).unwrap();
    let sc_acc_after = blockchain_mock.accounts.get(&sc_addr).unwrap();

    assert_eq!(user_acc_after.egld_balance, num_bigint::BigUint::from(0u32));
    assert_eq!(
        sc_acc_after.egld_balance,
        num_bigint::BigUint::from(3_000u32)
    );
}

/*
Update cache before call?

tx_context.tx_cache.subtract_egld_balance(
        &tx_context.tx_input_box.from,
        &tx_context.tx_input_box.egld_value,
    );
    tx_context.tx_cache.increase_egld_balance(
        &tx_context.tx_input_box.to,
        &tx_context.tx_input_box.egld_value,
    );

    // TODO: temporary, will convert to explicit builtin function first
    for esdt_transfer in tx_context.tx_input_box.esdt_values.iter() {
        tx_context.tx_cache.transfer_esdt_balance(
            &tx_context.tx_input_box.from,
            &tx_context.tx_input_box.to,
            &esdt_transfer.token_identifier,
            esdt_transfer.nonce,
            &esdt_transfer.value,
        );
    }

    let tx_result = if !is_smart_contract_address(&tx_context.tx_input_box.to)
        || tx_context.tx_input_box.func_name.is_empty()
    {
        // direct EGLD transfer
        TxResult::empty()
    } else {
        execute_tx_context(tx_context.clone())
    };

    let blockchain_updates = tx_context.into_blockchain_updates();

    (tx_result, blockchain_updates)
*/

// fn type_test<A: VMApi>(_sc: ContractObj<A>) {}

fn into_api<CB: ContractBase>(sc_obj: CB) -> CB::Api {
    sc_obj.raw_vm_api()
}

/*
fn execute_test_tx<F: FnOnce(DebugApi)>(
    tx_input: TxInput,
    world: BlockchainMock,
    f: F,
) -> BlockchainMock {
    let rc_world = Rc::new(world);
    let api = DebugApi::new(tx_input, TxCache::new(rc_world));

    f(api);
    let bu = api.into_blockchain_updates();

    let mut world = Rc::try_unwrap(rc_world).unwrap();
    bu.apply(&mut world);
    world
}
*/
