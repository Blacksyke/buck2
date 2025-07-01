/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use buck2_core::buck2_env;

pub struct SandcastleScheduleType {
    schedule_type: Option<&'static str>,
}

// TODO iguridi: consolidate with buckconfig
impl SandcastleScheduleType {
    const SCHEDULE_TYPE_CONTINUOUS: &'static str = "continuous";
    const SCHEDULE_TYPE_DIFF: &'static str = "diff";

    pub fn new() -> buck2_error::Result<Self> {
        // Same as RE does https://fburl.com/code/sj13r130
        let schedule_type =
            if let Some(env) = buck2_env!("SCHEDULE_TYPE", applicability = internal)? {
                Some(env)
            } else {
                buck2_env!("SANDCASTLE_SCHEDULE_TYPE", applicability = internal)?
            };
        Ok(Self { schedule_type })
    }

    pub fn is_continuous(&self) -> bool {
        self.schedule_type == Some(Self::SCHEDULE_TYPE_CONTINUOUS)
    }

    pub fn is_some(&self) -> bool {
        self.schedule_type.is_some()
    }

    pub fn is_diff(&self) -> bool {
        self.schedule_type == Some(Self::SCHEDULE_TYPE_DIFF)
    }

    pub fn testing_new(schedule_type: &'static str) -> Self {
        Self {
            schedule_type: Some(schedule_type),
        }
    }

    pub fn testing_empty() -> Self {
        Self {
            schedule_type: None,
        }
    }
}
