// TODO: Refactor: Errors

// [!] General Error: General Errors, such as incorrect input, etc. (used for user errors)
// [!!] Functional Error: Particular Function of Program Doesn't Work (used for internal errors)
// [!!!] Crucial Error: Program Termination (used for severe internal errors)

// TODO: use mongodb::error::Error as an example for this

pub const DATABASE_NOT_ASSIGNED_ERROR: &str = "DatabaseNotAssignedError";
pub const COLLECTION_NOT_ASSIGNED_ERROR: &str = "CollectionNotAssignedError";
pub const ELEMENT_NOT_FOUND_ERROR: &str = "ElementNotFoundEror";

#[derive(Debug)]
pub enum CoreError {
    BaseError,
    DBConnError,
    MongoDBError(mongodb::error::Error),
}

impl From<mongodb::error::Error> for CoreError {
    fn from(value: mongodb::error::Error) -> Self {
        Self::MongoDBError(value)
    }
}

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
