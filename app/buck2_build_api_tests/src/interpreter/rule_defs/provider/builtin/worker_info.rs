/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use buck2_build_api::interpreter::rule_defs::provider::registration::register_builtin_providers;
use buck2_interpreter_for_build::interpreter::testing::Tester;

fn run_info_tester() -> Tester {
    let mut tester = Tester::new().unwrap();
    tester.additional_globals(register_builtin_providers);
    tester
}

/// Test `id` is not there.
#[test]
fn run_display() {
    let mut tester = run_info_tester();
    tester
        .run_starlark_bzl_test(
            r#"
def test():
    assert_eq('WorkerInfo(exe=cmd_args("x"), concurrency=None, streaming=None, supports_bazel_remote_persistent_worker_protocol=False)', str(WorkerInfo(exe="x")))
"#,
        )
        .unwrap();
}
