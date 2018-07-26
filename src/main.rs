#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![feature(termination_trait_lib)]
#![feature(process_exitcode_placeholder)]
#![deny(unused_must_use)]

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod builtins;
mod error;
mod interpreter;
mod namespace;
mod scanner;
mod value;

use std::process::ExitCode;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
        println!("Usage: microforth [--version | --help] [filename]");
        return ExitCode::SUCCESS;
    }

    if args.contains(&"-V".to_owned()) || args.contains(&"--version".to_owned()) {
        println!("MicroForth {}", VERSION);
        return ExitCode::SUCCESS;
    }

    let fileargs: Vec<String> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with("-"))
        .cloned()
        .collect();
    let interactive = fileargs.is_empty()
        || args.contains(&"-i".to_owned())
        || args.contains(&"--interactive".to_owned());

    let mut interp = interpreter::Interpreter::new().with_builtins();

    for filepath in fileargs.iter() {
        let mut f = File::open(filepath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Could not read");

        if let Err(error) = interp.execute(&contents.to_owned(), Some(filepath)) {
            println!("Error: {:?}", error);
            return ExitCode::FAILURE;
        }
    }

    if !interactive {
        return ExitCode::SUCCESS;
    }

    println!("MicroForth {}", VERSION);
    println!(
        "Stack item size: {} bytes",
        std::mem::size_of::<value::Value>()
    );

    loop {
        println!("> ");
        return ExitCode::SUCCESS;
    }
}
