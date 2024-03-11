// consider using
// use thiserror::Error;

// TODO: use mongodb::error::Error as an example for this

pub const DATABASE_NOT_ASSIGNED_ERROR: &str = "DatabaseNotAssignedError";
pub const COLLECTION_NOT_ASSIGNED_ERROR: &str = "CollectionNotAssignedError";
pub const ELEMENT_NOT_FOUND_ERROR: &str = "ElementNotFoundEror";

#[derive(Debug)]
pub enum Error {
    BaseError,
    DBConnError,
    MongoDBError(mongodb::error::Error),
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Self::MongoDBError(value)
    }
}
