use std::{io::{ stdout, Write }, env};

use carlback_libs::{parse_args, ParseError, print_author};

fn main() {
    match parse_args(env::args().len(), env::args()) {
        Ok(args) => {
            let err = interpret_args(args);
            match err {
                Ok(_) => {},
                Err(_) => {
                    print_usage();
                }
            }
        },
        Err(err) => {
            match err {
                ParseError::NoArguments => {
                    handle_buffer_clear(true);
                },
                _ => {
                    println!("clear: Fatal Error: {:?}", err);
                }
            }
        }
    }
}
fn interpret_args(args: Vec<String>) -> Result<(), ParseError> {
    let mut clear_everything = true;
    for arg in args {
        match arg.as_str() {
            "-x" => {
                clear_everything = false;
            },
            "-h" => {
                print_usage();
                return Result::Ok(());
            },
            "-v" => {
                println!("clear - Version: 0.1.0");
                print_author();
                return Result::Ok(());
            },
            _ => {
                return Result::Err(ParseError::InvalidArgument(arg));
            }
        }
    }
    handle_buffer_clear(clear_everything);
    Ok(())
}
fn print_usage() {
    println!("Clears the output buffer using ANSI escape codes \\033c");
    print_author();
    println!("Usage: clear [OPTION]...");
    println!("Clears the output buffer using ANSI escape codes \\033c\n");
    println!("  -x\t\tDo not try to clear scrollback using \\033[2J\\033[1;1H");
    println!("  \t\tthis adds a line to the buffer and forces the cursor to the top of the output buffer");
    println!("  -h\t\tPrints help information");
    println!("  -v\t\tPrints version information");
}
fn handle_buffer_clear(clear_everything: bool) {
    let mut stout = stdout().lock();
    if clear_everything {
        write!(stout,"{esc}c", esc = 27 as char).expect("Could not clear entire buffer to the standard output") // kill the buffer with fire and brimstone
    } else {
        write!(stout, "{esc}[2J{esc}[1;1H", esc = 27 as char).expect("Could not write new buffer section to the standard output.") // kill the buffer *softly* with fire and brimstone
    }
    drop(stout)
}