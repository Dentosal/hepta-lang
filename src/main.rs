#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]
#![allow(unreachable_pub)]
#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod interpreter;
mod scanner;
mod stack;
mod value;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
        println!("Usage: microforth [--version | --help] [filename]");
        return;
    }

    if args.contains(&"-V".to_owned()) || args.contains(&"--version".to_owned()) {
        println!("MicroForth {}", VERSION);
        return;
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

    let mut interp = interpreter::Interpreter::new().with_builins();

    for filepath in fileargs.iter() {
        let mut f = File::open(filepath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Could not read");

        println!("{}", contents);

        interp.execute(&contents.to_owned(), Some(filepath));
    }

    if !interactive {
        return;
    }

    println!("MicroForth {}", VERSION);
    println!(
        "Stack item size: {} bytes",
        std::mem::size_of::<value::Value>()
    );

    loop {
        println!("> ");
        break;
    }
}
