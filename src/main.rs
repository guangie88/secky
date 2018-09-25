#![deny(missing_debug_implementations, warnings)]

extern crate atty;
#[macro_use]
extern crate failure;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use atty::Stream;
use std::io::Read;
use std::{error::Error, process};
use structopt::StructOpt;

type MainResult = std::result::Result<(), Box<Error>>;

/// Help keying in secrets into file or piped programs.
/// 
/// CTRL-D to end the text input, auto-trimming is performed unless trim flags
/// are set.
#[derive(StructOpt, Debug)]
#[structopt(name = "secky")]
struct Config {
    /// Disable trimming of whitespaces at the beginning
    #[structopt(short = "b", long = "no-trim-begin")]
    no_trim_begin: bool,

    /// Disable trimming of whitespaces at the end
    #[structopt(short = "e", long = "no-trim-end")]
    no_trim_end: bool,

    /// Force allow echoing to TTY for stdout
    #[structopt(short = "f", long = "force")]
    force_stdout_tty: bool,
}

fn run() -> MainResult {
    let config = Config::from_args();

    // validation
    let is_stdout_tty = atty::is(Stream::Stdout);

    if is_stdout_tty && !config.force_stdout_tty {
        Err(format_err!(
            "Stdout TTY is not allowed. Redirect stdout or use -f."
        ))?
    }

    // secret entry (CTRL-D to send EOF)
    // #[macro_use]
    // extern crate text_io;
    // let secret: String = try_read!("{}<<EOF")?;

    let mut secret = String::new();
    std::io::stdin().read_to_string(&mut secret)?;

    let secret = if config.no_trim_begin {
        &secret
    } else {
        secret.trim_left()
    };

    let secret = if config.no_trim_end {
        &secret
    } else {
        secret.trim_right()
    };

    print!("{}", secret);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
