// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
use fail::FailScenario;
use move_binary_format::file_format::empty_module;
use move_bytecode_verifier::VerifierConfig;
use move_core_types::{state::VMState, vm_status::StatusCode};

#[test]
fn test_unwind() {
    let scenario = FailScenario::setup();
    fail::cfg("verifier-failpoint-panic", "panic").unwrap();

    let m = empty_module();
    let res = move_bytecode_verifier::verify_module_with_config(
        &VerifierConfig {
            max_loop_depth: Some(5),
            max_generic_instantiation_length: Some(32),
            max_function_parameters: Some(128),
            max_basic_blocks: Some(1024),
            max_value_stack_size: 1024,
            max_type_nodes: Some(256),
            max_push_size: Some(10000),
            max_dependency_depth: 100,
        },
        &m,
    )
    .unwrap_err();
    assert_eq!(res.major_status(), StatusCode::VERIFIER_INVARIANT_VIOLATION);
    scenario.teardown();
}
