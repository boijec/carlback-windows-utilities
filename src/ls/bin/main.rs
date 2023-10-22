use std::{fs::read_dir, os::windows::prelude::MetadataExt, env};

mod list_source_core;
use list_source_core::{PrintMode, print_usage, interpret_args};
use carlback_libs::{parse_args, ParseError};

fn main() {
    match parse_args(env::args().len(), env::args()) {
        Ok(args) => {
            let err = interpret_args(args);
            match err {
                Ok(flags) => {
                    for flag in flags {
                        println!("{}", flag);
                    }
                },
                Err(_) => {
                    print_usage();
                }
            }
        },
        Err(err) => {
            match err {
                ParseError::NoArguments => {
                    handle_filesystem(".", false, PrintMode::Simple);
                },
                _ => {
                    println!("ls: Fatal Error: {:?}", err);
                }
            }
        }
    }
    // let (usize, Box<dyn Vec<Option<String>>>) = parse_args(env::args().len(), env::args());
    // if usize == 1 {
    //     println!("ls: Fatal_Error: Cannot use more than one display mode at a time.");
    //     return;
    // }
    // let print_all = matches.get_flag("print-all");
    // let force_list = matches.get_one::<String>("list-view");
    // let force_comma_view = matches.get_one::<String>("comma-view");
    // let location = matches.get_one::<String>("location").unwrap();
    
    // handle_filesystem(&location, print_all, PrintMode::from_char('a'));
}

// TODO: Add support for color printing on directories
// TODO: Add support for detailed list view
fn handle_filesystem(path: &str, print_all: bool, print_mode: PrintMode) { // fixing later, big mega-function go brrrr...
    let paths = match read_dir(path) {
        Ok(paths) => paths,
        Err(e) => {
            println!("ls: Error: {}", e);
            return;
        }
    };
    println!();
    let mut list_count = 0;
    // TODO: Implement counter behavior.. maybe the -m flag first for printng comma separated?
    for path in paths {
        if path.is_err() {
            println!("ls: Fatal_Error: Could not read path");
            break;
        }
        let p = path.unwrap();
        let meta = match p.metadata() {
            Ok(meta) => meta,
            Err(e) => {
                println!("ls: Fatal_Error: failed to unwrap metadata with err: {}", e);
                break;
            }
        };
        let attr = meta.file_attributes();
        if (attr & 0x2 > 0) && !print_all { // 0x2 byte means the file is hidden.
            continue;
        }
        let mut scaped_path = format!("{}", p.path().display());
        scaped_path = scaped_path.split("\\").last().unwrap().to_string();
        if scaped_path.starts_with(".") && !print_all { // aaand unix does not print files or dirs starting with a dot.
            continue;
        }
        match print_mode {
            PrintMode::Simple => {
                print!("{} ", scaped_path);
            },
            PrintMode::CommaArray => {
                if list_count == 0 {
                    list_count += 1;
                    print!("{}", scaped_path);
                } else {
                    print!(", {}", scaped_path);
                }
            }
            PrintMode::List => todo!(),
            PrintMode::Detailed => todo!(),
        }
    }
    println!();
}