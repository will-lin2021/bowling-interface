pub mod database;

#[cfg(test)]
mod database_tests {
    use crate::database::DatabaseConn;

    #[test]
    fn init_and_stop() {
        let mut conn = DatabaseConn::init();

        assert!(conn.stop());
    }
}
