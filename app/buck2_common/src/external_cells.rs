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

use async_trait::async_trait;
use buck2_core::cells::cell_root_path::CellRootPath;
use buck2_core::cells::external::ExternalCellOrigin;
use buck2_core::cells::name::CellName;
use buck2_util::late_binding::LateBinding;
use dice::DiceComputations;

use crate::file_ops::delegate::FileOpsDelegate;

#[async_trait]
pub trait ExternalCellsImpl: Send + Sync + 'static {
    async fn get_file_ops_delegate(
        &self,
        ctx: &mut DiceComputations<'_>,
        cell_name: CellName,
        origin: ExternalCellOrigin,
    ) -> buck2_error::Result<Arc<dyn FileOpsDelegate>>;

    fn check_bundled_cell_exists(&self, cell_name: CellName) -> buck2_error::Result<()>;

    async fn expand(
        &self,
        ctx: &mut DiceComputations<'_>,
        cell_name: CellName,
        origin: ExternalCellOrigin,
        path: &CellRootPath,
    ) -> buck2_error::Result<()>;
}

pub static EXTERNAL_CELLS_IMPL: LateBinding<&'static dyn ExternalCellsImpl> =
    LateBinding::new("EXTERNAL_CELLS_IMPL");
