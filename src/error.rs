pub enum Error {
    Info(String),    // [!]
    Warning(String), // [!!]
    Severe(String),  // [!!!]
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info(i) => {
                write!(f, "[!] {}", i)
            }
            Self::Warning(w) => {
                write!(f, "[!!] {}", w)
            }
            Self::Severe(s) => {
                write!(f, "[!!!] {}", s)
            }
        }
    }
}
