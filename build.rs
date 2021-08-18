//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use vergen::{vergen, Config, TimestampKind};

fn main() {
    let mut config = Config::default();

    if vergen(config).is_err() {
        *config.git_mut().enabled_mut() = false;
    } else {
        *config.git_mut().commit_timestamp_kind_mut() = TimestampKind::DateOnly;
    }

    vergen(config).expect("Failed to generate version information")
}
