use clap::{Arg, Command, ArgAction};

fn main() {
    let matches = Command::new("Clear Buffer")
        .version("0.1.0")
        .author("Carl \"Callback\" Boije <boijec@gmail.com>")
        .about("Clears the output buffer using ANSI escape codes \\033c")
        .arg(Arg::new("dont-clear-scroll")
            .short('x')
            .required(false)
            .action(ArgAction::SetFalse)
            .help("Do not try to clear scrollback using \\033[2J\\033[1;1H\nthis adds a line to the buffer and forces the cursor to the top of the output buffer")
        )
        .get_matches();
    let clear_everything = matches.get_flag("dont-clear-scroll");
    handle_buffer_clear(clear_everything);
}

fn handle_buffer_clear(clear_everything: bool) {
    if clear_everything {
        print!("{esc}c", esc = 27 as char) // kill the buffer with fire and brimstone
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char) // kill the buffer *softly* with fire and brimstone
    }
}