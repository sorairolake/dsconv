//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

mod cli;
mod value;

use std::fs;
use std::io::{self, Read};

use anyhow::{bail, Result};
use structopt::StructOpt;

use crate::cli::Opt;

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let input = match opt.input {
        Some(ref f) => fs::read_to_string(f)?,
        None if atty::isnt(atty::Stream::Stdin) => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
        _ => bail!("Input from tty is invalid."),
    };

    let opt = opt.process();

    match opt.output {
        Some(f) => fs::write(f, input)?,
        None => print!("{}", input),
    }

    Ok(())
}
