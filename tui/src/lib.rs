#![allow(clippy::or_fun_call)]
mod app;

use std::ffi::OsString;
use std::time::Duration;

use radicle_common::args::{Args, Error, Help};

pub const TICK_RATE: u64 = 200;

pub const HELP: Help = Help {
    name: "tui",
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    usage: r#"
Usage

    rad tui

Options
    --help              Print help
"#,
};

#[derive(Default, Debug)]
pub struct Options {
    verbose: bool,
}

impl Args for Options {
    fn from_args(args: Vec<OsString>) -> anyhow::Result<(Self, Vec<OsString>)> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_args(args);
        let mut verbose = false;

        while let Some(arg) = parser.next()? {
            match arg {
                Long("verbose") | Short('v') => {
                    verbose = true;
                }
                Long("help") => {
                    return Err(Error::Help.into());
                }
                _ => return Err(anyhow::anyhow!(arg.unexpected())),
            }
        }

        

        Ok((
            Options {
                verbose,
            },
            vec![],
        ))
    }
}

pub fn run(_options: Options) -> anyhow::Result<()> {
    let tick_rate = Duration::from_millis(TICK_RATE);
    app::exec(tick_rate)?;
    
    Ok(())
}
