use std::thread;

use surrealdb;

pub struct DatabaseConn {
    server_instance: Option<std::thread::JoinHandle<bool>>,
}

impl DatabaseConn {
    pub fn init() -> Self {
        let db_thread_builder = thread::Builder::new().name("SurrealDB".to_string());

        DatabaseConn {
            server_instance: db_thread_builder.spawn(|| {
                eprintln!("{:}", thread::current().name().is_some());

                true
            }).ok(),
        }
    }

    pub fn stop(&mut self) -> bool {
        self.server_instance
            .take().expect("Called stop on non-running thread")
            .join().expect("Unable to join thread")
    }
}
