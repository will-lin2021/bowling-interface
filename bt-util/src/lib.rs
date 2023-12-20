pub mod database;

pub mod testing_items {
    use mysql::{params, FromRowError, Params};

    #[derive(Debug)]
    pub struct TestStruct {
        pub date: chrono::NaiveDate,
        pub num: u8,
    }

    impl std::cmp::PartialEq for TestStruct {
        fn eq(&self, other: &Self) -> bool {
            self.date == other.date && self.num == other.num
        }
    }

    impl mysql::prelude::FromRow for TestStruct {
        fn from_row(row: mysql_common::Row) -> Self
        where
            Self: Sized,
        {
            match (row.get(0), row.get(1)) {
                (Some(date), Some(num)) => TestStruct { date, num },
                _ => panic!(),
            }
        }

        fn from_row_opt(row: mysql_common::Row) -> Result<Self, mysql_common::FromRowError>
        where
            Self: Sized,
        {
            match (row.get(0), row.get(1)) {
                (Some(date), Some(num)) => Ok(TestStruct { date, num }),
                _ => Err(FromRowError(row)),
            }
        }
    }

    impl super::database::DBInsert for TestStruct {
        fn ins_statement(&self, table: &str) -> String {
            format!("INSERT INTO {table} (date, num) VALUES (:date, :num)")
        }

        fn ins_parameter(&self) -> Params {
            params! {
                "date" => self.date,
                "num" => self.num,
            }
        }
    }

    impl super::database::DBModify for TestStruct {
        fn mod_statement(&self, table: &str) -> String {
            format!("UPDATE {table} SET num=:num WHERE date=:date")
        }

        fn mod_parameter(&self) -> Params {
            params! {
                "date" => self.date,
                "num" => self.num,
            }
        }
    }

    impl super::database::DBQuery for TestStruct {
        fn query_statement(&self, table: &str, sort: bool) -> String {
            if sort {
                format!("SELECT * FROM {table} WHERE date=:date ORDER BY num")
            } else {
                format!("SELECT * FROM {table} WHERE date=:date")
            }
        }

        fn query_parameter(&self) -> Params {
            params! {
                "date" => self.date,
                "num" => self.num,
            }
        }
    }

    impl super::database::DBDelete for TestStruct {
        fn del_statement(&self, table: &str) -> String {
            format!("DELETE FROM {table} WHERE (date, num) = (:date, :num)")
        }

        fn del_parameter(&self) -> Params {
            params! {
                "date" => self.date,
                "num" => self.num,
            }
        }
    }
}

#[cfg(test)]
mod database_tests {
    use super::database::DBConn;
    use super::testing_items::TestStruct;

    use std::env;

    use chrono::NaiveDate;
    use dotenv::dotenv;

    #[test]
    fn serial_tests() {
        let mut conn = db_connect();

        db_insert(&mut conn);
        db_modify(&mut conn);
        db_query(&mut conn);
        db_query_sort(&mut conn);
        db_delete(&mut conn);

        db_add_rm_dup(&mut conn);

        db_system(&mut conn);

        conn.wipe().unwrap();
    }

    fn db_connect() -> DBConn<TestStruct> {
        dotenv().ok();

        let mut conn = match DBConn::connect(
            env::var("DB_USER").ok(),
            env::var("DB_PASS").ok(),
            env::var("DB_IP").ok(),
            env::var("DB_PORT").unwrap().parse().unwrap(),
            env::var("DB_TEST_DB").ok(),
            env::var("DB_TEST_TABLE").ok().unwrap(),
        ) {
            Ok(conn) => conn,
            Err(err) => panic!("{err}"),
        };

        match conn.wipe() {
            Ok(()) => (),
            Err(err) => panic!("{err}"),
        }

        conn
    }

    fn db_insert(conn: &mut DBConn<TestStruct>) {
        // Initialize entries
        let entry1 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            num: 1,
        };
        let entry2 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            num: 2,
        };

        // Insert entries
        assert!(conn.insert(&entry1).is_ok());
        match conn.query(&entry1) {
            Ok(vec) => assert_eq!(vec.len(), 1),
            Err(err) => panic!("Query failed with err {err}"),
        };
        assert!(conn.insert(&entry2).is_ok());
        match conn.query(&entry2) {
            Ok(vec) => assert_eq!(vec.len(), 2),
            Err(err) => panic!("Query failed with err {err}"),
        };

        // Query entries
        let result = match conn.query(&entry1) {
            Ok(vec) => vec,
            Err(err) => panic!("{err}"),
        };
        match result.get(0) {
            Some(result) => assert_eq!(result, &entry1),
            None => panic!("Result doesn't match original struct"),
        };
        match result.get(1) {
            Some(result) => assert_eq!(result, &entry2),
            None => panic!("Result doesn't match original struct"),
        };
    }

    fn db_modify(conn: &mut DBConn<TestStruct>) {
        // Initialize entries
        let origin = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
            num: 1,
        };
        let replacement = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
            num: 2,
        };

        // Insert origin entry
        conn.insert(&origin).unwrap();

        // Modify original entry
        match conn.modify(&replacement) {
            Ok(()) => (),
            Err(err) => panic!("Modify failed with err {err}"),
        };

        // Check entry for modification
        let query = conn.query(&replacement).unwrap();
        let query_val = query.get(0).unwrap();
        assert_ne!(query_val, &origin);
        assert_eq!(query_val, &replacement);
    }

    fn db_query(conn: &mut DBConn<TestStruct>) {
        // Initialize entries
        let entry1 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 3, 3).unwrap(),
            num: 1,
        };
        let entry2 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 3, 3).unwrap(),
            num: 3,
        };
        let entry3 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 3, 3).unwrap(),
            num: 2,
        };
        let entry4 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 3, 3).unwrap(),
            num: 5,
        };
        let entry5 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 3, 3).unwrap(),
            num: 4,
        };

        // Insert entries
        conn.insert(&entry1).unwrap();
        conn.insert(&entry2).unwrap();
        conn.insert(&entry3).unwrap();
        conn.insert(&entry4).unwrap();
        conn.insert(&entry5).unwrap();

        // Query entries
        let result = match conn.query(&entry1) {
            Ok(vec) => vec,
            Err(err) => panic!("Query failed with err {err}"),
        };
        assert_eq!(result.len(), 5);
        assert_eq!(result.get(0), Some(&entry1));
        assert_eq!(result.get(1), Some(&entry2));
        assert_eq!(result.get(2), Some(&entry3));
        assert_eq!(result.get(3), Some(&entry4));
        assert_eq!(result.get(4), Some(&entry5));
    }

    fn db_query_sort(conn: &mut DBConn<TestStruct>) {
        // Initialize entries
        let entry1 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 4, 4).unwrap(),
            num: 1,
        };
        let entry2 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 4, 4).unwrap(),
            num: 3,
        };
        let entry3 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 4, 4).unwrap(),
            num: 2,
        };
        let entry4 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 4, 4).unwrap(),
            num: 5,
        };
        let entry5 = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 4, 4).unwrap(),
            num: 4,
        };

        // Insert entries
        conn.insert(&entry1).unwrap();
        conn.insert(&entry2).unwrap();
        conn.insert(&entry3).unwrap();
        conn.insert(&entry4).unwrap();
        conn.insert(&entry5).unwrap();

        // Query entries
        let result = match conn.query_sort(&entry1) {
            Ok(vec) => vec,
            Err(err) => panic!("Query failed with err {err}"),
        };
        assert_eq!(result.len(), 5);
        assert_eq!(result.get(0), Some(&entry1));
        assert_eq!(result.get(1), Some(&entry3));
        assert_eq!(result.get(2), Some(&entry2));
        assert_eq!(result.get(3), Some(&entry5));
        assert_eq!(result.get(4), Some(&entry4));
    }

    fn db_delete(conn: &mut DBConn<TestStruct>) {
        // Initialize entry
        let entry = TestStruct {
            date: NaiveDate::from_ymd_opt(2020, 5, 5).unwrap(),
            num: 1,
        };

        // Insert entry
        conn.insert(&entry).unwrap();

        // Verify entry existence
        match conn.query(&entry) {
            Ok(vec) => {
                assert_eq!(vec.len(), 1);
                assert_eq!(vec.get(0), Some(&entry));
            }
            Err(err) => panic!("Query failed with err {err}"),
        }

        // Delete entry
        match conn.delete(&entry) {
            Ok(()) => (),
            Err(err) => panic!("Delete failed with err {err}"),
        }

        // Verify entry deletion
        match conn.query(&entry) {
            Ok(vec) => {
                assert_eq!(vec.len(), 0);
                assert_eq!(vec.get(0), None);
            }
            Err(err) => panic!("Query failed with err {err}"),
        }
    }

    fn db_add_rm_dup(conn: &mut DBConn<TestStruct>) {
        let entry = TestStruct {
            date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            num: 1,
        };

        conn.insert(&entry).unwrap();
        conn.insert(&entry).unwrap();

        match conn.query(&entry) {
            Ok(vec) => {
                assert_eq!(vec.len(), 2);
                assert_eq!(vec.get(0), Some(&entry));
                assert_eq!(vec.get(0), vec.get(1));
            }
            Err(err) => panic!("Query failed with err {err}"),
        }

        conn.delete(&entry).unwrap();

        match conn.query(&entry) {
            Ok(vec) => {
                assert_eq!(vec.len(), 0);
                assert_eq!(vec.get(0), None);
                assert_eq!(vec.get(1), None);
            }
            Err(err) => panic!("Query failed with err {err}"),
        }
    }

    fn db_system(conn: &mut DBConn<TestStruct>) {
        // TODO: Full System TEST
    }
}
