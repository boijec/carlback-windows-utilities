use std::str::Chars;

use carlback_libs::{print_author, ParseError};

pub enum PrintMode {
    List,
    CommaArray,
    Detailed,
    Simple
}
pub enum ViewMode {
    AllFiles,
    StandardFiles
}

impl PrintMode {
    pub fn from_char(c: char) -> PrintMode {
        match c {
            'l' => PrintMode::List,
            'm' => PrintMode::CommaArray,
            'd' => PrintMode::Detailed,
            _ => PrintMode::Simple
        }
    }
}
impl ViewMode {
    pub fn from_char(c: char) -> ViewMode {
        match c {
            'a' => ViewMode::AllFiles,
            _ => ViewMode::StandardFiles
        }
    }
}
// this feels retarded, and it is... this should be moved out one step and this method should just take care of the char options... shouldn't have switched from clap to this...
pub fn interpret_args(args: Vec<String>) -> Result<Vec<char>, ParseError> {
    let argc = args.len();
    let mut argp = args.into_iter();
    let mut flags: Vec<char> = Vec::new();
    
    loop {
        if let Some(arg) = argp.next() {
            if arg.starts_with("-") && arg.len() > 1 {
                let mut chars = arg.chars();
                chars.next();
                for c in chars {
                    match c {
                        'a' => {
                            flags.push(c);
                        },
                        'l' => {
                            flags.push(c)
                        },
                        'm' => {
                            if !flags.contains(&'l') {
                                flags.push(c)
                            }
                        },
                        'h' => {
                            print_usage();
                            return Result::Ok(Vec::new());
                        },
                        'v' => {
                            println!("ls - Version: 0.1.0");
                            print_author();
                            return Result::Ok(Vec::new());
                        },
                        _ => {
                            return Result::Err(ParseError::InvalidOptionValue(arg));
                        }
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if flags.len() > 0 {
        return Result::Ok(flags);
    }
    Result::Err(ParseError::NoArguments)
}
pub fn print_usage() {
    println!("List source information about the FILEs (the current directory by default).");
    println!("Usage: ls [OPTION]... [FILE]...");
    print_author();
    println!("Arguments:");
    println!("  [FILE]  [default: .]");
    println!();
    println!("Options:");
    println!("  -a\t\tShows all files, including hidden files");
    println!("  -l\t\tForcibly formats files into list view");
    println!("  -m\t\tForcibly formats files into a comma separated array");
    println!("  -h\t\tPrints help information");
    println!("  -v\t\tPrints version information");
}