// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

mod genesis_context;
pub mod genesis_gas_schedule;

use serde::{Deserialize, Serialize};
use std::env;

use crate::{genesis_context::GenesisStateView, genesis_gas_schedule::INITIAL_GAS_SCHEDULE};
use diem_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    PrivateKey, Uniform,
};
use diem_framework_releases::{
    current_module_blobs, legacy::transaction_scripts::LegacyStdlibScript,
};
use diem_transaction_builder::stdlib as transaction_builder;
use diem_types::{
    account_address,
    account_config::{
        self,
        events::CreateAccountEvent,
    },
    chain_id::ChainId,
    contract_event::ContractEvent,
    on_chain_config::{VMPublishingOption, DIEM_MAX_KNOWN_VERSION},
    transaction::{
        authenticator::AuthenticationKey, ChangeSet, ScriptFunction, Transaction, WriteSetPayload,
    },
};
use diem_vm::{convert_changeset_and_events, data_cache::StateViewCache};
use move_core_types::{
    account_address::AccountAddress,
    gas_schedule::{CostTable, GasAlgebra, GasUnits},
    identifier::Identifier,
    language_storage::{ModuleId, StructTag, TypeTag},
    value::{serialize_values, MoveValue},
};
use move_vm_runtime::{
    logging::{LogContext, NoContextLog},
    move_vm::MoveVM,
    session::Session,
};
use move_vm_types::gas_schedule::{zero_cost_schedule, CostStrategy};
use once_cell::sync::Lazy;
use rand::prelude::*;
use transaction_builder::encode_create_designated_dealer_script_function;
use vm::CompiledModule;

// The seed is arbitrarily picked to produce a consistent key. XXX make this more formal?
const GENESIS_SEED: [u8; 32] = [42; 32];

const GENESIS_MODULE_NAME: &str = "Genesis";
const DIEM_VERSION_MODULE_NAME: &str = "DiemVersion";

pub static GENESIS_KEYPAIR: Lazy<(Ed25519PrivateKey, Ed25519PublicKey)> = Lazy::new(|| {
    let mut rng = StdRng::from_seed(GENESIS_SEED);
    let private_key = Ed25519PrivateKey::generate(&mut rng);
    let public_key = private_key.public_key();
    (private_key, public_key)
});

pub static ZERO_COST_SCHEDULE: Lazy<CostTable> = Lazy::new(zero_cost_schedule);

const ZERO_AUTH_KEY: [u8; 32] = [0; 32];

pub type Name = Vec<u8>;
//////// 0L ////////
// Defines a validator owner and maps that to an operator
pub type OperatorAssignment = 
    (Option<Ed25519PublicKey>, Name, ScriptFunction, GenesisMiningProof);

//////// 0L ////////
// Defines a validator operator and maps that to a validator (config)
pub type OperatorRegistration = (Ed25519PublicKey, Name, ScriptFunction, AccountAddress);

pub fn encode_genesis_transaction(
    diem_root_key: Option<&Ed25519PublicKey>,            //////// 0L ////////
    treasury_compliance_key: Option<&Ed25519PublicKey>,  //////// 0L ////////
    operator_assignments: &[OperatorAssignment],
    operator_registrations: &[OperatorRegistration],
    vm_publishing_option: Option<VMPublishingOption>,
    chain_id: ChainId,
) -> Transaction {
    Transaction::GenesisTransaction(WriteSetPayload::Direct(encode_genesis_change_set(
        diem_root_key,
        treasury_compliance_key,
        operator_assignments,
        operator_registrations,
        current_module_blobs(), // Must use compiled stdlib,
        //////// 0L ////////        
        vm_publishing_option
            .unwrap_or_else(|| VMPublishingOption::open()), // :)
        chain_id,
    )))
}

pub fn encode_genesis_change_set(
    diem_root_key: Option<&Ed25519PublicKey>,           //////// 0L ////////
    treasury_compliance_key: Option<&Ed25519PublicKey>, //////// 0L ////////
    operator_assignments: &[OperatorAssignment],
    operator_registrations: &[OperatorRegistration],
    stdlib_modules: &[Vec<u8>],
    vm_publishing_option: VMPublishingOption,
    chain_id: ChainId,
) -> ChangeSet {
    let mut stdlib_module_tuples: Vec<(ModuleId, &Vec<u8>)> = Vec::new();
    // create a data view for move_vm
    let mut state_view = GenesisStateView::new();
    for module in stdlib_modules {
        let module_id = CompiledModule::deserialize(module).unwrap().self_id();
        state_view.add_module(&module_id, &module);
        stdlib_module_tuples.push((module_id, module));
    }
    let data_cache = StateViewCache::new(&state_view);

    let move_vm = MoveVM::new();
    let mut session = move_vm.new_session(&data_cache);
    let log_context = NoContextLog::new();

    //////// 0L ////////
    let xdx_ty = TypeTag::Struct(StructTag {
        address: *account_config::GAS_MODULE.address(),
        module: account_config::GAS_MODULE.name().to_owned(),
        name: account_config::GAS_STRUCT_NAME.to_owned(),
        type_params: vec![],
    });

    create_and_initialize_main_accounts(
        &mut session,
        &log_context,
        diem_root_key,
        treasury_compliance_key,
        vm_publishing_option,
        &xdx_ty,
        chain_id,
    );
    //////// 0L ////////
    println!("OK create_and_initialize_main_accounts =============== ");

    let genesis_env = get_env();
    println!("Initializing with env: {}", genesis_env);
    if genesis_env != "prod"  {
        initialize_testnet(&mut session, &log_context);
    }    
    //////// 0L end ////////

    // generate the genesis WriteSet
    create_and_initialize_owners_operators(
        &mut session,
        &log_context,
        &operator_assignments,
        &operator_registrations,
    );
    //////// 0L ////////
    println!("OK create_and_initialize_owners_operators =============== ");

    distribute_genesis_subsidy(&mut session, &log_context);
    println!("OK Genesis subsidy =============== ");
    //////// 0L end ////////

    reconfigure(&mut session, &log_context);

    //////// 0L ////////
    // if [NamedChain::TESTNET, NamedChain::DEVNET, NamedChain::TESTING]
    //     .iter()
    //     .any(|test_chain_id| test_chain_id.id() == chain_id.id())
    // {
    //     create_and_initialize_testnet_minting(
    //         &mut session, &log_context, &treasury_compliance_key.unwrap()   //////// 0L ////////
    //     );
    // }

    let (mut changeset1, mut events1) = session.finish().unwrap();

    let state_view = GenesisStateView::new();
    let data_cache = StateViewCache::new(&state_view);
    let mut session = move_vm.new_session(&data_cache);
    publish_stdlib(&mut session, &log_context, stdlib_module_tuples);
    let (changeset2, events2) = session.finish().unwrap();

    changeset1.squash(changeset2).unwrap();
    events1.extend(events2);

    let (write_set, events) = convert_changeset_and_events(changeset1, events1).unwrap();

    assert!(!write_set.iter().any(|(_, op)| op.is_deletion()));
    verify_genesis_write_set(&events);
    ChangeSet::new(write_set, events)
}

fn exec_function(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
    module_name: &str,
    function_name: &str,
    ty_args: Vec<TypeTag>,
    args: Vec<Vec<u8>>,
) {
    session
        .execute_function(
            &ModuleId::new(
                account_config::CORE_CODE_ADDRESS,
                Identifier::new(module_name).unwrap(),
            ),
            &Identifier::new(function_name).unwrap(),
            ty_args,
            args,
            &mut CostStrategy::system(&ZERO_COST_SCHEDULE, GasUnits::new(100_000_000)),
            log_context,
        )
        .unwrap_or_else(|e| {
            panic!(
                "Error calling {}.{}: {}",
                module_name,
                function_name,
                e.into_vm_status()
            )
        });
}

fn exec_script_function(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
    sender: AccountAddress,
    script_function: &ScriptFunction,
) {
    session
        .execute_script_function(
            script_function.module(),
            script_function.function(),
            script_function.ty_args().to_vec(),
            script_function.args().to_vec(),
            vec![sender],
            &mut CostStrategy::system(&ZERO_COST_SCHEDULE, GasUnits::new(100_000_000)),
            log_context,
        )
        .unwrap()
}

//////// 0L ////////
/// Create and initialize Association and Core Code accounts.
fn create_and_initialize_main_accounts(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
    diem_root_key: Option<&Ed25519PublicKey>,
    _treasury_compliance_key: Option<&Ed25519PublicKey>,
    publishing_option: VMPublishingOption,
    xdx_ty: &TypeTag,
    chain_id: ChainId,
) {
    let diem_root_auth_key:AuthenticationKey;
    if diem_root_key.is_some() {
        diem_root_auth_key = AuthenticationKey::ed25519(&diem_root_key.unwrap());
    } else {
        diem_root_auth_key = AuthenticationKey::new(
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        );
    }   

    // let treasury_compliance_auth_key = AuthenticationKey::ed25519(treasury_compliance_key);

    let root_diem_root_address = account_config::diem_root_address();
    //////// 0L ////////
    // let tc_account_address = account_config::treasury_compliance_account_address();

    let initial_allow_list = MoveValue::Vector(
        publishing_option
            .script_allow_list
            .into_iter()
            .map(|hash| MoveValue::vector_u8(hash.to_vec().into_iter().collect()))
            .collect(),
    );

    let genesis_gas_schedule = &INITIAL_GAS_SCHEDULE;
    let instr_gas_costs = bcs::to_bytes(&genesis_gas_schedule.instruction_table)
        .expect("Failure serializing genesis instr gas costs");
    let native_gas_costs = bcs::to_bytes(&genesis_gas_schedule.native_table)
        .expect("Failure serializing genesis native gas costs");

    exec_function(
        session,
        log_context,
        GENESIS_MODULE_NAME,
        "initialize",
        vec![],
        serialize_values(&vec![
            MoveValue::Signer(root_diem_root_address),
            // MoveValue::Signer(tc_account_address), //////// 0L ////////
            MoveValue::vector_u8(diem_root_auth_key.to_vec()),
            //////// 0L ////////
            // MoveValue::vector_u8(treasury_compliance_auth_key.to_vec()), 
            initial_allow_list,
            MoveValue::Bool(publishing_option.is_open_module),
            MoveValue::vector_u8(instr_gas_costs),
            MoveValue::vector_u8(native_gas_costs),
            MoveValue::U8(chain_id.id()),
        ]),
    );

    // Bump the Diem Framework version number
    exec_function(
        session,
        log_context,
        DIEM_VERSION_MODULE_NAME,
        "set",
        vec![],
        serialize_values(&vec![
            MoveValue::Signer(root_diem_root_address),
            MoveValue::U64(
                /* Diem Framework major version number */ DIEM_MAX_KNOWN_VERSION.major,
            ),
        ]),
    );

    // Bump the sequence number for the Association account. If we don't do this and a
    // subsequent transaction (e.g., minting) is sent from the Assocation account, a problem
    // arises: both the genesis transaction and the subsequent transaction have sequence
    // number 0
    exec_function(
        session,
        log_context,
        "DiemAccount",
        "epilogue",
        vec![xdx_ty.clone()],
        serialize_values(&vec![
            MoveValue::Signer(root_diem_root_address),
            MoveValue::U64(/* txn_sequence_number */ 0),
            MoveValue::U64(/* txn_gas_price */ 0),
            MoveValue::U64(/* txn_max_gas_units */ 0),
            MoveValue::U64(/* gas_units_remaining */ 0),
        ]),
    );
}

// 0L todo: Unused fn
fn _create_and_initialize_testnet_minting( //////// 0L ////////
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
    public_key: &Ed25519PublicKey,
) {
    let genesis_auth_key = AuthenticationKey::ed25519(public_key);
    let create_dd_script = encode_create_designated_dealer_script_function(
        account_config::xus_tag(),
        0,
        account_config::testnet_dd_account_address(),
        genesis_auth_key.prefix().to_vec(),
        b"moneybags".to_vec(), // name
        true,                  // add_all_currencies
    )
    .into_script_function();

    let mint_max_xus = transaction_builder::encode_tiered_mint_script_function(
        account_config::xus_tag(),
        0,
        account_config::testnet_dd_account_address(),
        std::u64::MAX / 2,
        3,
    )
    .into_script_function();

    // Create the DD account
    exec_script_function(
        session,
        log_context,
        account_config::treasury_compliance_account_address(),
        &create_dd_script,
    );

    // mint XUS.
    let treasury_compliance_account_address = account_config::treasury_compliance_account_address();
    exec_script_function(
        session,
        log_context,
        treasury_compliance_account_address,
        &mint_max_xus,
    );

    let testnet_dd_account_address = account_config::testnet_dd_account_address();
    exec_script_function(
        session,
        log_context,
        testnet_dd_account_address,
        &transaction_builder::encode_rotate_authentication_key_script_function(
            genesis_auth_key.to_vec(),
        )
        .into_script_function(),
    );
}

//////// 0L ////////
/// Creates and initializes each validator owner and validator operator. This method creates all
/// the required accounts, sets the validator operators for each validator owner, and sets the
/// validator config on-chain.
fn create_and_initialize_owners_operators(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
    operator_assignments: &[OperatorAssignment],
    operator_registrations: &[OperatorRegistration],
) {
    let diem_root_address = account_config::diem_root_address();

    // Create accounts for each validator owner. The inputs for creating an account are the auth
    // key prefix and account address. Internally move then computes the auth key as auth key
    // prefix || address. Because of this, the initial auth key will be invalid as we produce the
    // account address from the name and not the public key.
    println!("0 ======== Create Owner Accounts");    
    for (owner_key, owner_name, _op_assignment, genesis_proof) in operator_assignments {
        // TODO: Remove. Temporary Authkey for genesis, because accounts are being created from human names. 
        let staged_owner_auth_key = 
            AuthenticationKey::ed25519(owner_key.as_ref().unwrap());
        let owner_address = staged_owner_auth_key.derived_address();
        dbg!(owner_address);
        // let staged_owner_auth_key = diem_config::utils::default_validator_owner_auth_key_from_name(owner_name);
        //TODO: why does this need to be derived from human name?
        // let owner_address = staged_owner_auth_key.derived_address();
        let create_owner_script =
            transaction_builder::encode_create_validator_account_script_function(
                0,
                owner_address,
                staged_owner_auth_key.prefix().to_vec(),
                owner_name.clone(),
            )
            .into_script_function();
        exec_script_function(
            session,
            log_context,
            diem_root_address,
            &create_owner_script,
        );

        // If there is a key, make it the auth key, otherwise use a zero auth key.
        let real_owner_auth_key = if let Some(owner_key) = owner_key {
            AuthenticationKey::ed25519(owner_key).to_vec()
        } else {
            ZERO_AUTH_KEY.to_vec() // TODO: is this used for tests?
        };

        // Rotate auth key.
        exec_script_function(
            session,
            log_context,
            owner_address.clone(),
            &transaction_builder::encode_rotate_authentication_key_script_function(
                real_owner_auth_key,
            )
            .into_script_function(),
        );

        // Submit mining proof
        let preimage = hex::decode(&genesis_proof.preimage).unwrap();
        let proof = hex::decode(&genesis_proof.proof).unwrap();
        exec_function(
            session,
            log_context,
            "MinerState",
            "genesis_helper",
            vec![],
            serialize_values(&vec![
                MoveValue::Signer(diem_root_address),                
                MoveValue::Signer(owner_address),                
                MoveValue::vector_u8(preimage),
                MoveValue::vector_u8(proof)
            ])
        );

        exec_function(
            session,
            log_context,
            "ValidatorUniverse",
            "genesis_helper",
            vec![],
            serialize_values(&vec![
                MoveValue::Signer(diem_root_address),                
                MoveValue::Signer(owner_address),
            ])
        );

        exec_function(
            session,
            log_context,
            "FullnodeState",
            "init",
            vec![],
            serialize_values(&vec![
                MoveValue::Signer(owner_address),
            ])
        );        
    }

    println!("1 ======== Create OP Accounts");
    // Create accounts for each validator operator
    for (operator_key, operator_name, _, _) in operator_registrations {
        let operator_auth_key = AuthenticationKey::ed25519(&operator_key);
        let operator_account = account_address::from_public_key(operator_key);
        let create_operator_script =
            transaction_builder::encode_create_validator_operator_account_script_function(
                0,
                operator_account,
                operator_auth_key.prefix().to_vec(),
                operator_name.clone(),
            )
            .into_script_function();
        exec_script_function(
            session,
            log_context,
            diem_root_address,
            &create_operator_script,
        );
    }

    println!("2 ======== Link owner to OP");    
    // Authorize an operator for a validator/owner
    for (owner_key, _owner_name, op_assignment_script, _genesis_proof) in operator_assignments {
        // let owner_address = diem_config::utils::validator_owner_account_from_name(owner_name);
        let staged_owner_auth_key = AuthenticationKey::ed25519(owner_key.as_ref().unwrap());
        let owner_address = staged_owner_auth_key.derived_address();
        exec_script_function(session, log_context, owner_address, op_assignment_script);
    }
    
    println!("3 ======== OP sends network info to Owner config");
    // Set the validator operator configs for each owner
    for (operator_key, _, registration, _account) in operator_registrations {
        let operator_account = account_address::from_public_key(operator_key);
        exec_script_function(session, log_context, operator_account, registration);
    }

    println!("4 ======== Add owner to validator set");
    // Add each validator to the validator set
    for (owner_key, _owner_name, _op_assignment, _genesis_proof) in operator_assignments {
        let staged_owner_auth_key = AuthenticationKey::ed25519(owner_key.as_ref().unwrap());
        let owner_address = staged_owner_auth_key.derived_address();
        // let owner_address = diem_config::utils::validator_owner_account_from_name(owner_name);
        exec_function(
            session,
            log_context,
            "DiemSystem",
            "add_validator",
            vec![],
            serialize_values(&vec![
                MoveValue::Signer(diem_root_address),
                MoveValue::Address(owner_address),
            ]),
        );
    }
}

/// Publish the standard library.
fn publish_stdlib(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
    stdlib: Vec<(ModuleId, &Vec<u8>)>,
) {
    let genesis_removed = stdlib
        .iter()
        .filter(|(module_id, _bytes)| module_id.name().as_str() != GENESIS_MODULE_NAME);
    for (module_id, bytes) in genesis_removed {
        assert!(module_id.name().as_str() != GENESIS_MODULE_NAME);
        session
            .publish_module(
                (*bytes).clone(),
                *module_id.address(),
                &mut CostStrategy::system(&ZERO_COST_SCHEDULE, GasUnits::new(100_000_000)),
                log_context,
            )
            .unwrap_or_else(|e| panic!("Failure publishing module {:?}, {:?}", module_id, e));
    }
}

/// Trigger a reconfiguration. This emits an event that will be passed along to the storage layer.
fn reconfigure(session: &mut Session<StateViewCache>, log_context: &impl LogContext) {
    exec_function(
        session,
        log_context,
        "DiemConfig",
        "emit_genesis_reconfiguration_event",
        vec![],
        vec![],
    );
}

/// Verify the consistency of the genesis `WriteSet`
fn verify_genesis_write_set(events: &[ContractEvent]) {
    // (1) first event is account creation event for DiemRoot
    let create_diem_root_event = &events[0];
    assert_eq!(
        *create_diem_root_event.key(),
        CreateAccountEvent::event_key(),
    );

    // (2) second event is account creation event for TreasuryCompliance
    let create_treasury_compliance_event = &events[1];
    assert_eq!(
        *create_treasury_compliance_event.key(),
        CreateAccountEvent::event_key(),
    );

    //////// 0L ////////
    // // (3) The first non-account creation event should be the new epoch event
    // let new_epoch_events: Vec<&ContractEvent> = events
    //     .iter()
    //     .filter(|e| e.key() == &NewEpochEvent::event_key())
    //     .collect();
    // assert!(
    //     new_epoch_events.len() == 1,
    //     "There should only be one NewEpochEvent"
    // );
    // // (4) This should be the first new_epoch_event
    // assert_eq!(new_epoch_events[0].sequence_number(), 0,);
}

/// An enum specifying whether the compiled stdlib/scripts should be used or freshly built versions
/// should be used.
#[derive(Debug, Eq, PartialEq)]
pub enum GenesisOptions {
    Compiled,
    Fresh,
}

/// Generate an artificial genesis `ChangeSet` for testing
pub fn generate_genesis_change_set_for_testing(genesis_options: GenesisOptions) -> ChangeSet {
    let modules = match genesis_options {
        GenesisOptions::Compiled => diem_framework_releases::current_module_blobs(),
        GenesisOptions::Fresh => diem_framework::module_blobs(),
    };

    generate_test_genesis(modules, VMPublishingOption::open(), None).0
}

pub fn test_genesis_transaction() -> Transaction {
    let changeset = test_genesis_change_set_and_validators(None).0;
    Transaction::GenesisTransaction(WriteSetPayload::Direct(changeset))
}

pub fn test_genesis_change_set_and_validators(count: Option<usize>) -> (ChangeSet, Vec<Validator>) {
    generate_test_genesis(
        &current_module_blobs(),
        VMPublishingOption::locked(LegacyStdlibScript::allowlist()),
        count,
    )
}

pub struct Validator {
    pub index: usize,
    pub key: Ed25519PrivateKey,
    pub oper_key: Ed25519PrivateKey, //////// 0L ////////
    pub name: Vec<u8>,
    pub operator_address: AccountAddress,
    pub owner_address: AccountAddress,
}

impl Validator {
    pub fn new_set(count: Option<usize>) -> Vec<Validator> {
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([1u8; 32]);
        (0..count.unwrap_or(4)) //////// 0L ////////
            .map(|idx| Validator::gen(idx, &mut rng))
            .collect()
    }

    fn gen(index: usize, rng: &mut rand::rngs::StdRng) -> Self {
        let name = index.to_string().as_bytes().to_vec();
        let key = Ed25519PrivateKey::generate(rng);
        let oper_key = Ed25519PrivateKey::generate(rng);
        let operator_address = account_address::from_public_key(&oper_key.public_key());
        let owner_address = account_address::from_public_key(&key.public_key());

        Self {
            index,
            key,
            oper_key, //////// 0L ////////
            name,
            operator_address,
            owner_address,
        }
    }

    fn operator_assignment(&self) -> OperatorAssignment {
        let script_function = transaction_builder::encode_set_validator_operator_script_function(
            self.name.clone(),
            self.operator_address,
        )
        .into_script_function();
        (
            Some(self.key.public_key()),
            self.name.clone(),
            script_function,
            //////// 0L ////////
            GenesisMiningProof::default() // NOTE: For testing only
        )
    }

    fn operator_registration(&self) -> OperatorRegistration {
        let script_function =
            transaction_builder::encode_register_validator_config_script_function(
                self.owner_address,
                self.key.public_key().to_bytes().to_vec(),
                bcs::to_bytes(&[0u8; 0]).unwrap(),
                bcs::to_bytes(&[0u8; 0]).unwrap(),
            )
            .into_script_function();
        (
            self.oper_key.public_key(),
            self.name.clone(),
            script_function,
            self.operator_address, //////// 0L ////////
        )
    }
}

pub fn generate_test_genesis(
    stdlib_modules: &[Vec<u8>],
    vm_publishing_option: VMPublishingOption,
    count: Option<usize>,
) -> (ChangeSet, Vec<Validator>) {
    let validators = Validator::new_set(count);

    let genesis = encode_genesis_change_set(
        Some(&GENESIS_KEYPAIR.1), //////// 0L ////////
        Some(&GENESIS_KEYPAIR.1), //////// 0L ////////
        &validators
            .iter()
            .map(|v| v.operator_assignment())
            .collect::<Vec<_>>(),
        &validators
            .iter()
            .map(|v| v.operator_registration())
            .collect::<Vec<_>>(),
        stdlib_modules,
        vm_publishing_option,
        ChainId::test(),
    );
    (genesis, validators)
}

//////// 0L ////////
/// Genesis subsidy to miners
fn distribute_genesis_subsidy(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext,
) { 
    let diem_root_address = account_config::diem_root_address();

    exec_function(
        session,
        log_context,
        "Subsidy",
        "genesis",
        vec![],
        serialize_values(&vec![MoveValue::Signer(diem_root_address)])
    )
}

//////// 0L ////////
fn get_env() -> String {
    match env::var("NODE_ENV") {
        Ok(val) => val,
        _ => "test".to_string() // default to "test" if not set
    }
}

//////// 0L ////////
// 0L Change: Necessary for genesis transaction.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenesisMiningProof {
    pub preimage: String,
    pub proof: String,
}

//////// 0L ////////
impl Default for GenesisMiningProof {
    fn default() -> GenesisMiningProof {

        // These use "alice" fixtures from ../fixtures and used elsewhere in the project, in both easy(stage) and hard(Prod) mode.
        //TODO: These fixtures should be moved to /fixtures/miner_fixtures.rs

        let easy_preimage = "87515d94a244235a1433d7117bc0cb154c613c2f4b1e67ca8d98a542ee3f59f5000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000304c20746573746e65746400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865206e6174696f6e".to_owned();

        let easy_proof = "002c4dc1276a8a58ea88fc9974c847f14866420cbc62e5712baf1ae26b6c38a393c4acba3f72d8653e4b2566c84369601bdd1de5249233f60391913b59f0b7f797f66897de17fb44a6024570d2f60e6c5c08e3156d559fbd901fad0f1343e0109a9083e661e5d7f8c1cc62e815afeee31d04af8b8f31c39a5f4636af2b468bf59a0010f48d79e7475be62e7007d71b7355944f8164e761cd9aca671a4066114e1382fbe98834fe32cf494d01f31d1b98e3ef6bffa543928810535a063c7bbf491c472263a44d9269b1cbcb0aa351f8bd894e278b5d5667cc3f26a35b9f8fd985e4424bedbb3b77bdcc678ccbb9ed92c1730dcdd3a89c1a8766cbefa75d6eeb7e5921000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001".to_owned();

        //NOTE: this is same as easy_preimage
        // let hard_preimage = easy_preimage.to_owned();
        let hard_preimage = "87515d94a244235a1433d7117bc0cb154c613c2f4b1e67ca8d98a542ee3f59f5000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000304c20746573746e6574404b4c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000050726f74657374732072616765206163726f737320746865206e6174696f6e".to_owned();

        let hard_proof =  "001725678f78425dac39e394fc07698dd8fc891dfba0822cecc5d21434dacde903f508c1e12844eb4b97a598653cc6d03524335edf51b43f090199288488b537fd977cc5f53069f609a2f758f121e887f28f0fc1150aa5649255f8b7caea9edf6228640358d1a4fe43ddb6ad6ce1c3a6a28166e2f0b7e7310e80bfbb1db85e096000065a89b7f44ebc495d70db6034fd529a80e0b5bb74ace62cffb89f4e16e54f93e4a0063ca3651dd8486b466607973a51aacb0c66213e64e0b7bf291c64d81ed4a517a0abe58da4ae46f6191c808d9ba7c636cee404ed02248794db3fab6e5e4ab517f6f3fa12f39fb88fb5a143b5d9c16a31e3c3e173deb11494f792b52a67a70034a065c665b1ef05921a6a8ac4946365d61b2b4d5b86a607ba73659863d774c3fc7c2372f5b6c8b5ae068d4e20aac5e42b501bf441569d377f70e8f87db8a6f9b1eadb813880dbeb89872121849df312383f4d8007747ae76e66e5a13d9457af173ebb0c5eb9c39ee1ac5cef94aa75e1d5286349c88051c36507960de1f37377ffddc80a66578b437ac2a6d04fc7a595075b978bd844919d03ffe9db5b6440b753273c498aa2a139de42188d278d1ce1e3ddfdd99a97a64907e1cdf30d1c55dfc7262cd3175eb1f268ee2a91576fcd6bd644031413f55e42c510d08a81e747de36c0a6c9019d219571ea6851f43a551d6012a5317cc52992a72c270c1570419665".to_owned();

        if get_env() == "test"  {
            return GenesisMiningProof {
                preimage: easy_preimage,
                proof: easy_proof,
            }

        } else {
            return GenesisMiningProof {
                preimage: hard_preimage,
                proof: hard_proof,
            }
        }
    }
}

//////// 0L ////////
fn initialize_testnet(
    session: &mut Session<StateViewCache>,
    log_context: &impl LogContext
) {
    let diem_root_address = account_config::diem_root_address();
    let mut module_name = "Testnet";
    if get_env() == "stage" { 
        module_name = "StagingNet";
    };
    exec_function(
        session,
        log_context,
        module_name,
        "initialize",
        vec![],
        serialize_values(&vec![MoveValue::Signer(diem_root_address)])
    );
}