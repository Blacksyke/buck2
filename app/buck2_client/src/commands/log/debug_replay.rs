/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use buck2_client_ctx::client_ctx::ClientCommandContext;
use buck2_client_ctx::common::BuckArgMatches;
use buck2_client_ctx::events_ctx::EventsCtx;
use buck2_client_ctx::exit_result::ExitResult;
use buck2_core::soft_error;

use crate::commands::log::replay::ReplayCommand;

#[derive(Debug, buck2_error::Error)]
#[buck2(tag = Input)]
enum DebugReplayCommandError {
    #[error("`buck2 debug replay` is deprecated. Use `buck2 log replay` instead.")]
    Deprecated,
}

#[derive(clap::Parser, Debug)]
pub struct DebugReplayCommand {
    #[clap(flatten)]
    replay: ReplayCommand,
}

impl DebugReplayCommand {
    pub(crate) fn exec(
        self,
        matches: BuckArgMatches<'_>,
        ctx: ClientCommandContext<'_>,
        events_ctx: &mut EventsCtx,
    ) -> ExitResult {
        soft_error!(
            "debug_replay",
            DebugReplayCommandError::Deprecated.into(),
            deprecation: true
        )?;
        ctx.exec(self.replay, matches, events_ctx)
    }
}
