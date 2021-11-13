//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

#[macro_export]
macro_rules! long_version {
    () => {{
        static LONG_VERSION: once_cell::sync::OnceCell<String> = once_cell::sync::OnceCell::new();

        LONG_VERSION.get_or_init(|| {
            let head = if let (Some(sha_short), Some(commit_date)) = (
                option_env!("VERGEN_GIT_SHA_SHORT"),
                option_env!("VERGEN_GIT_COMMIT_DATE"),
            ) {
                format!(
                    "{} ({} {})",
                    env!("CARGO_PKG_VERSION"),
                    sha_short,
                    commit_date
                )
            } else {
                env!("CARGO_PKG_VERSION").to_string()
            };
            format!(
                "{}\n\n{}\n{}\n\n{}",
                head,
                "Copyright (C) 2021 Shun Sakai",
                "License: Apache License 2.0",
                "Report bugs to https://github.com/sorairolake/dsconv/issues"
            )
        })
    }};
}
