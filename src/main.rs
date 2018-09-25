#![deny(missing_debug_implementations, warnings)]

extern crate atty;
#[macro_use]
extern crate failure;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate text_io;

use atty::Stream;
use std::{error::Error, process};
use structopt::StructOpt;

type MainResult = std::result::Result<(), Box<Error>>;

/// Help keying in secrets into file or piped programs
#[derive(StructOpt, Debug)]
#[structopt(name = "secky")]
struct Config {
    /// EOF chars to use to terminate the stdin entry
    #[structopt(short = "e", long = "eof", default_value = "<<EOF")]
    eof_chars: String,

    /// Force allow echoing to TTY for stdout
    #[structopt(short = "f", long = "force")]
    force_stdout_tty: bool,
}

fn run() -> MainResult {
    let config = Config::from_args();

    // validation
    let is_stdout_tty = atty::is(Stream::Stdout);

    if is_stdout_tty && !&config.force_stdout_tty {
        Err(format_err!(
            "Stdout TTY is not allowed. Redirect stdout or use -f."
        ))?
    }

    // secret entry
    let secret: String = try_read!("{}<<EOF")?;
    print!("{}", secret);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
