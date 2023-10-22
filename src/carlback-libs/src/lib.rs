#[derive(Debug)]
pub enum ParseError { // my dumbass will probably need this in the future..
    NoArguments,
    TooManyArguments,
    InvalidArgument(String),
    InvalidFlag(String),
    InvalidOption(String),
    InvalidOptionValue(String),
    InvalidOptionType(String),
    InvalidOptionCount(String),
    Other
}

// Simple and easy way to force all args into a vector of strings.
pub fn parse_args(argc: usize, argv: std::env::Args) -> Result<Vec<String>, ParseError> {
    if argc < 2 {
        return Result::Err(ParseError::NoArguments);
    }
    return Result::Ok(argv.skip(1).collect());
}
pub fn print_author() {
    println!("Author: Carl \"Callback\" Boije <boijec@gmail.com>");
}