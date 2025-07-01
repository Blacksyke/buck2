/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::sync::Arc;

use allocative::Allocative;
use buck2_core::build_file_path::BuildFilePath;

use crate::oncall::Oncall;

/// Package-specific data for `TargetNode`.
///
/// (Note this has nothing to do with `PACKAGE` files which are not implemented
/// at the moment of writing.)
#[derive(Debug, Hash, Allocative, Eq, PartialEq)]
pub struct Package {
    /// The build file which defined this target, e.g. `fbcode//foo/bar/TARGETS`
    pub buildfile_path: Arc<BuildFilePath>,
    /// The oncall attribute, if set
    pub oncall: Option<Oncall>,
}
