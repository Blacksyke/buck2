/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

//! Builtin providers.

pub mod configuration_info;
pub mod constraint_setting_info;
pub mod constraint_value_info;
pub mod default_info;
pub mod dep_only_incompatible_info;
pub mod dep_only_incompatible_rollout;
pub mod execution_platform_info;
pub mod execution_platform_registration_info;
pub mod external_runner_test_info;
pub mod install_info;
pub mod local_resource_info;
pub mod platform_info;
pub mod run_info;
pub mod template_placeholder_info;
pub mod ty;
pub mod validation_info;
pub mod worker_info;
pub mod worker_run_info;
