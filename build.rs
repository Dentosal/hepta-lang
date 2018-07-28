#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]
#![allow(unreachable_pub)]
#![allow(dead_code)]
#![feature(termination_trait_lib)]
#![feature(process_exitcode_placeholder)]
#![deny(unused_must_use)]
#![allow(unknown_lints)]
#![warn(clippy_pedantic)]
#![allow(match_wild_err_arm)]
#![allow(unused_extern_crates)]
#![allow(similar_names)]

#[macro_use]
extern crate serde_derive;
extern crate crc;
extern crate regex;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::time::SystemTime;

use regex::Regex;

const RS_FN_SIGNATURE: &str = "(interp: &mut Interpreter) -> Result<(), Error>";
const RS_POP_ARG: &str = "interp.data.pop().ok_or(Error::StackUndeflow)?";
const RE_RUST_FN: &str =
    r"pub\s*fn\s*([a-zA-Z][a-zA-Z0-9_]*)\((.*?)\)\s*(->\s*(.+?)\s*)?\{";

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    pub rust_src_crc32: u32,
    pub build_rs_crc32: u32,
    pub modify_time_st: SystemTime,
}
impl Header {
    pub fn eq_crc32s(&self, other: &Self) -> bool {
        self.rust_src_crc32 == other.rust_src_crc32
            && self.build_rs_crc32 == other.build_rs_crc32
    }
}

fn format_rust(src_text: String) -> String {
    let mut p = Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let stdin_bytes: Vec<u8> = src_text.bytes().collect();
    {
        let stdin = p.stdin.as_mut().expect("Failed to get stdin");
        stdin
            .write_all(stdin_bytes.as_slice())
            .expect("Failed to write to stdin");
    }

    let output = p.wait_with_output().expect("Failed to wait on child");

    String::from_utf8(output.stdout).expect("Output: utf-8 required")
}

fn get_own_file() -> String {
    let mut file = File::open("build.rs").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn get_target_file_mtime(filepath: &str) -> Option<SystemTime> {
    let file = File::open(filepath).ok()?;
    file.metadata().ok()?.modified().ok()
}

fn get_target_file_header(filepath: &str) -> Option<Header> {
    let mut file = File::open(format!("{}.cache", filepath)).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    let header: Header = serde_json::from_str(contents.trim()).ok()?;
    Some(header)
}

fn set_target_file_header(filepath: &str, mut header: Header) {
    header.modify_time_st = match get_target_file_mtime(filepath) {
        Some(mt) => mt,
        None => panic!("File modification time could not be read"),
    };
    let header_json = serde_json::to_string(&header).unwrap();
    let header_json_bytes: Vec<u8> = header_json.bytes().collect();

    let mut file = File::create(format!("{}.cache", filepath)).unwrap();
    file.write_all(header_json_bytes.as_slice()).unwrap();
}

fn get_rust_srcfile(filepath: &str) -> String {
    let p = Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(p.stdout).expect("Output: utf-8 required");
    let sysroot = stdout.trim();

    let path = format!("{}{}", sysroot, filepath);
    let mut file = File::open(path).expect("No rust source found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(Debug)]
struct Function {
    pub name: String,
    pub arg_types: Vec<String>,
    pub ret_type: String,
}
impl Function {
    pub fn argc(&self) -> usize {
        self.arg_types.len() + 1
    }
    pub fn fmt_signature(&self) -> String {
        let mut argtypes = vec!["u64".to_owned()];
        for at in &self.arg_types {
            argtypes.push(at.clone());
        }
        let ats = argtypes.join(", ");
        format!("`{}({}) -> {}`", self.name, ats, self.ret_type)
    }
}

fn get_functions(src_text: &str) -> Vec<Function> {
    let start = src_text.find("uint_impl").unwrap();
    let until = src_text.find("impl u8 {").unwrap();

    let re = Regex::new(RE_RUST_FN).unwrap();

    let mut result: Vec<Function> = Vec::new();
    for re_match in re.find_iter(src_text[start..until].as_ref()) {
        for cap in re.captures_iter(re_match.as_str()) {
            let name: &str = cap[1].trim();
            let args: Vec<&str> = cap[2].split(',').map(|a| a.trim()).collect();
            let ret_type: String = cap[4].trim().to_owned();
            if ret_type.is_empty() {
                panic!("Empty return type not allowed (f: {})", name);
            }

            if name.starts_with("to_") {
                continue;
            }

            if ret_type.starts_with('[') {
                continue;
            }

            if args.is_empty() || args[0] != "self" {
                continue;
            }

            let args: Vec<(&str, &str)> = args
                .iter()
                .skip(1)
                .map(|arg| {
                    let mut halfs = arg.splitn(2, ':').map(|a| a.trim());
                    (halfs.next().unwrap(), halfs.next().unwrap())
                })
                .collect();

            if args.iter().any(|(k, _)| k.starts_with("mut ")) {
                continue;
            }

            let arg_types: Vec<String> =
                args.iter().map(|(_, v)| v.clone().to_owned()).collect();

            result.push(Function {
                name: name.to_owned(),
                arg_types,
                ret_type,
            })
        }
    }
    result
}

fn type_checked_start(argindex: usize) -> String {
    format!(
        "if let Value::Integer(a{}) = arg{} {{\n",
        argindex, argindex
    )
}

fn type_checked_end(argindex: usize, required_type: &str) -> String {
    let mut r_text = String::new();
    r_text.push_str("} else {\n");
    r_text.push_str("    Err(Error::WrongArgumentType(\n");
    r_text.push_str(format!("        arg{}.type_(),\n", argindex).as_ref());
    r_text.push_str(format!("        vec![ValueType::{}],\n", required_type).as_ref());
    r_text.push_str("    ))\n");
    r_text.push_str("}\n");

    r_text
}

fn generate_int_functions(src_text: &str) -> String {
    let src_text: String = src_text.replace("Self", "u64");

    // header block
    let mut file_text: String = "// !!! DO NOT EDIT !!!\n".to_owned();
    file_text.push_str("// All manual changes will be lost!\n");
    file_text.push_str(format!("// Autogenerated by {}\n\n", file!()).as_ref());

    // std use statements
    file_text.push_str("use std::convert::TryFrom;\n\n");

    // local use statements
    file_text.push_str(
        "use crate::error::Error;
        use crate::interpreter::Interpreter;
        use crate::value::{BuiltinFunction, Value, ValueType};\n\n",
    );

    let functions = get_functions(src_text.as_ref());

    for f in &functions {
        // doc comment
        let mut fn_text = format!("/// {}\n", f.fmt_signature());

        // signature
        fn_text.push_str(format!("fn f_{}{} {{\n", f.name, RS_FN_SIGNATURE).as_ref());

        // pop args
        for i in (0..f.argc()).rev() {
            fn_text.push_str(format!("let arg{} = {};\n", i, RS_POP_ARG).as_ref());
        }

        // type_check
        fn_text.push_str(&type_checked_start(0));
        for i in (1..f.argc()).rev() {
            fn_text.push_str(&type_checked_start(i));
        }

        // convert inputs
        fn_text.push_str("let a0_conv = a0;\n");
        for (i, atype) in f.arg_types.iter().enumerate() {
            let conv_stmt = match atype.as_ref() {
                "u64" => format!("a{}", i + 1),
                "u32" => format!(
                    "u32::try_from(a{}).map_err(|_| Error::IntegerOverflow)?",
                    i + 1
                ),
                _ => panic!("Invalid arg type"),
            };

            fn_text.push_str(format!("let a{}_conv = {};\n", i + 1, conv_stmt).as_ref());
        }

        // do the actual operation
        fn_text.push_str(format!("let r: {} = u64::{}(\n", f.ret_type, f.name).as_ref());
        for i in 0..f.argc() {
            fn_text.push_str(format!("a{}_conv,\n", i).as_ref());
        }
        fn_text.push_str(");\n");

        // return type conversion & push
        fn_text.push_str(match f.ret_type.as_ref() {
            "u64" => "interp.data.push(Value::Integer(r));\n",
            "u32" => "interp.data.push(Value::Integer(u64::from(r)));\n",
            "bool" => "interp.data.push(Value::Boolean(r));\n",
            "Option<u64>" => {
                "if let Some(v) = r {
                    interp.data.push(Value::Integer(v));
                }
                interp.data.push(Value::Boolean(r.is_some()));\n"
            },
            "(u64, bool)" => {
                "interp.data.push(Value::Boolean(r.1));
                interp.data.push(Value::Integer(r.0));\n"
            },
            x => {
                panic!("Unhandled return type: {:?}", x);
            },
        });

        // ok return
        fn_text.push_str("Ok(())");

        // /type_check
        for (i, atype) in f.arg_types.iter().enumerate() {
            let arg_type = match atype.as_ref() {
                "u64" => "Integer",
                "u32" => "Integer",
                _ => panic!("Invalid arg type"),
            };
            fn_text.push_str(&type_checked_end(i + 1, arg_type));
        }
        fn_text.push_str(&type_checked_end(0, "Integer"));

        // end
        fn_text.push_str("}\n\n");

        file_text.push_str(fn_text.as_ref());
    }

    // register function

    file_text.push_str("pub fn register_all(interp: &mut Interpreter) {\n");
    for f in &functions {
        file_text.push_str(
            format!(
                "interp.register_builtin(BuiltinFunction::new(\"{}\", f_{}));",
                f.name, f.name,
            ).as_ref(),
        );
    }

    file_text.push_str("}\n");

    format_rust(file_text)
}

fn main() {
    let target_file_path = "src/builtins/generated/int.rs";
    let target_header = get_target_file_header(target_file_path);

    let src_text = get_rust_srcfile("/lib/rustlib/src/rust/src/libcore/num/mod.rs");
    let src_text_bytes: Vec<u8> = src_text.bytes().collect();
    let rust_src_crc32 = crc::crc32::checksum_ieee(&src_text_bytes);

    let own_text = get_own_file();
    let own_text_bytes: Vec<u8> = own_text.bytes().collect();
    let build_rs_crc32 = crc::crc32::checksum_ieee(&own_text_bytes);

    let header = Header {
        rust_src_crc32,
        build_rs_crc32,
        modify_time_st: SystemTime::now(), // just a placeholder
    };

    // check if already ok
    if let Some(th) = target_header {
        if header.eq_crc32s(&th) {
            if let Some(mt) = get_target_file_mtime(target_file_path) {
                if mt == th.modify_time_st {
                    println!("Using the existing version");
                    return;
                }
            }
        }
    }

    println!("Generating function code");

    let intf: String = generate_int_functions(src_text.as_ref());

    let intf_bytes: Vec<u8> = intf.bytes().collect();
    let mut file = File::create(target_file_path).unwrap();
    file.write_all(intf_bytes.as_slice()).unwrap();

    set_target_file_header(target_file_path, header);
}
