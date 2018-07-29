#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![feature(termination_trait_lib)]
#![feature(process_exitcode_placeholder)]
#![feature(euclidean_division)]
#![feature(try_from)]
#![feature(no_panic_pow)]
#![feature(reverse_bits)]
#![feature(wrapping_next_power_of_two)]
#![deny(unused_must_use)]
#![allow(unknown_lints)]
#![warn(clippy_pedantic)]
#![allow(match_wild_err_arm)]
#![allow(unused_extern_crates)]
#![allow(similar_names)]

extern crate dirs;
extern crate rustyline;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod builtins;
mod error;
mod interpreter;
mod namespace;
mod scanner;
mod value;

use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
        println!("Usage: hepta [--version | --help] [filename]");
        return ExitCode::SUCCESS;
    }

    if args.contains(&"-V".to_owned()) || args.contains(&"--version".to_owned()) {
        println!("Hepta {}", VERSION);
        return ExitCode::SUCCESS;
    }

    let fileargs: Vec<String> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with('-'))
        .cloned()
        .collect();
    let interactive = fileargs.is_empty()
        || args.contains(&"-i".to_owned())
        || args.contains(&"--interactive".to_owned());

    let mut interp = interpreter::Interpreter::new().with_builtins();

    for filepath in &fileargs {
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

    // REPL

    println!("Hepta {} REPL", VERSION);
    println!(
        "Stack item size: {} bytes",
        std::mem::size_of::<value::Value>(),
    );

    let histfile = {
        let mut path = dirs::home_dir().expect("No user home directory found");
        path.push(".hepta_history");
        path
    };

    let mut rl = Editor::<()>::new();
    if rl.load_history(&histfile).is_err() {
        println!("No previous history.");
    }

    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                if let Err(error) = interp.execute(line.as_ref(), None) {
                    println!("Error: {:?}", error);
                    continue;
                }
            },
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(_) => panic!("Unhandled readline error"),
        }
    }
    rl.save_history(&histfile).expect("Could not save histfile");;
    ExitCode::SUCCESS
}
