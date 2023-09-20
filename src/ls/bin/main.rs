use std::{fs::read_dir, os::windows::prelude::MetadataExt};

mod list_source_core;
use clap::{Arg, Command, ArgAction};
use list_source_core::PrintMode;

fn main() {
    let matches = Command::new("List Source")
        .version("0.1.0")
        .author("Carl \"Callback\" Boije <boijec@gmail.com>")
        .about("List source information about the FILEs (the current directory by default).")
        .arg(Arg::new("print-all")
            .short('a')
            .required(false)
            .action(ArgAction::SetTrue)
            .help("Shows all files, including hidden files")
        )
        .arg(Arg::new("list-view")
            .short('l')
            .required(false)
            .conflicts_with("comma-view")
            .action(ArgAction::SetTrue)
            .help("Forcibly formats files into list view")
        )
        .arg(Arg::new("comma-view")
            .short('m')
            .required(false)
            .conflicts_with("list-view")
            .action(ArgAction::SetTrue)
            .help("Forcibly formats files into a comma separated array")
        )
        .arg(Arg::new("location")
            .value_name("FILE")
            .default_value(".")
        )
        .get_matches();
    let print_all = matches.get_flag("print-all");
    let _force_list = matches.get_flag("list-view");
    let location = matches.get_one::<String>("location").unwrap();

    handle_filesystem(&location, print_all, PrintMode::from_char('a'));
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