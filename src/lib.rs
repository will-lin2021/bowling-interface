// TODO: This should contain all function necessary for running the interface
pub use std::env;

pub mod backend;
pub mod error;
pub mod loops;

pub mod app;

pub mod prelude {
    pub use crate::backend::core::types::*;

    // Add this as a feature, to allow for using different DB Connectors?
    // pub use crate::backend::database::mongodb_conn::DatabaseConn;
}

pub fn connect_db() -> Option<backend::database::mongodb_conn::DatabaseConn> {
    let user_string = match env::var("USER_NAME") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };
    let pass_string = match env::var("USER_PASS") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };
    let auth_string = match env::var("USER_AUTH") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };

    let host_string = match env::var("DB_HOST") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };
    let port_string = match env::var("DB_PORT") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };

    let db_string = match env::var("DB_NAME") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };
    let coll_string = match env::var("COLL_NAME") {
        Ok(string) => string,
        Err(_) => {
            return None;
        }
    };

    Some(backend::database::mongodb_conn::DatabaseConn::connect_full(
        &user_string,
        &pass_string,
        &host_string,
        &port_string,
        &auth_string,
        &db_string,
        &coll_string,
    ))
}
