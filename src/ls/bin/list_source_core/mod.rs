pub enum PrintMode {
    List,
    CommaArray,
    Detailed,
    Simple
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