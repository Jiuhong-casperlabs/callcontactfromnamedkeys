#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{NamedKeys, Parameters},
    CLType, CLValue, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    NamedKey, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn world() {
    runtime::put_key("inworld", storage::new_uref(1).into())
}

#[no_mangle]
pub extern "C" fn hello() {
    let contract_hash =
        ContractHash::from(runtime::get_key("worldhash").unwrap().into_hash().unwrap());
    runtime::call_contract(contract_hash, "world", RuntimeArgs::default())
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = {
        let mut entry_points = EntryPoints::new();

        let entry_point = EntryPoint::new(
            "world",
            Parameters::default(),
            CLType::String,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(entry_point);

        entry_points
    };
    let (contract_hash, _contract_version) = storage::new_contract(entry_points, None, None, None);

    let mut nameskeys = NamedKeys::new();
    nameskeys.insert("worldhash".to_string(), contract_hash.into());

    let entry_points = {
        let mut entry_points = EntryPoints::new();

        let entry_point = EntryPoint::new(
            "hello",
            Parameters::default(),
            CLType::String,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(entry_point);

        entry_points
    };
    let (contract_hash, _contract_version) =
        storage::new_contract(entry_points, Some(nameskeys), None, None);

    runtime::put_key("call", contract_hash.into());
    runtime::call_contract(contract_hash, "hello", RuntimeArgs::default())
}
