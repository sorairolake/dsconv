//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

fn generate_man_page(source: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> io::Result<()> {
    let status = match Command::new("asciidoctor")
        .args(["-a", concat!("manversion=", env!("CARGO_PKG_VERSION"))])
        .args(["-b", "manpage"])
        .args(["-D".as_ref(), out_dir.as_ref()])
        .arg(source.as_ref())
        .status()
    {
        Ok(status) => status,
        Err(error) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to execute Asciidoctor: {}", error),
            ))
        }
    };

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Asciidoctor failed with the exit code: {}",
                status
                    .code()
                    .expect("The process was terminated by a signal")
            ),
        ))
    }
}

fn main() {
    let out_dir =
        env::var_os("OUT_DIR").expect("OUT_DIR is not defined as an environment variable");

    let current_dir = env::current_dir().expect("Failed to get the current working directory");
    let man_dir = current_dir.join("doc/man");
    let man_page_sources = [
        man_dir.join("man1/dsconv.1.adoc"),
        man_dir.join("man5/dsconv-config.toml.5.adoc"),
    ];

    for man_page_source in &man_page_sources {
        println!("cargo:rerun-if-changed={}", man_page_source.display());

        if let Err(error) = generate_man_page(man_page_source, &out_dir) {
            println!(
                "cargo:warning=Failed to generate a man page from {}: {}",
                man_page_source
                    .file_name()
                    .map(AsRef::as_ref)
                    .map(Path::display)
                    .expect("Failed to get the final component of a path"),
                error
            );
        }
    }
}
