use std::{collections::HashMap, env};

use dotenvy::dotenv;

use super::DatabaseConn;

#[test]
fn serial_tests() {
    dotenv().expect(".env not found");

    let mut conn_info: HashMap<&str, String> = HashMap::new();

    conn_info.insert("user", env::var("TEST_NAME").unwrap());
    conn_info.insert("pass", env::var("TEST_PASS").unwrap());
    conn_info.insert("auth", env::var("TEST_AUTH").unwrap());
    conn_info.insert("host", env::var("DB_HOST").unwrap());
    conn_info.insert("port", env::var("DB_PORT").unwrap());
    conn_info.insert("db", env::var("TEST_DB_NAME").unwrap());
    conn_info.insert("coll", env::var("TEST_COLL_NAME").unwrap());

    connect(&conn_info);
    connect_db(&conn_info);

    let db_conn = connect_full(&conn_info);

    set_database(&db_conn);
    unset_database(&db_conn);
    set_collection(&db_conn);
    unset_collection(&db_conn);

    add_game(&db_conn);
    add_games(&db_conn);
    get_game(&db_conn);
    get_games(&db_conn);
    num_games(&db_conn);
    modify_game(&db_conn);
    modify_games(&db_conn);
    remove_game(&db_conn);
    remove_games(&db_conn);
}

fn connect(conn_info: &HashMap<&str, String>) -> DatabaseConn {
    DatabaseConn::connect(
        conn_info.get("user").unwrap(),
        conn_info.get("pass").unwrap(),
        conn_info.get("host").unwrap(),
        conn_info.get("port").unwrap(),
        conn_info.get("auth").unwrap(),
    )
}

fn connect_db(conn_info: &HashMap<&str, String>) -> DatabaseConn {
    DatabaseConn::connect_db(
        conn_info.get("user").unwrap(),
        conn_info.get("pass").unwrap(),
        conn_info.get("host").unwrap(),
        conn_info.get("port").unwrap(),
        conn_info.get("auth").unwrap(),
        conn_info.get("db").unwrap(),
    )
}

fn connect_full(conn_info: &HashMap<&str, String>) -> DatabaseConn {
    DatabaseConn::connect_full(
        conn_info.get("user").unwrap(),
        conn_info.get("pass").unwrap(),
        conn_info.get("host").unwrap(),
        conn_info.get("port").unwrap(),
        conn_info.get("auth").unwrap(),
        conn_info.get("db").unwrap(),
        conn_info.get("coll").unwrap(),
    )
}

fn set_database(db_conn: &DatabaseConn) {
    todo!()
}

fn unset_database(db_conn: &DatabaseConn) {
    todo!()
}

fn set_collection(db_conn: &DatabaseConn) {
    todo!()
}

fn unset_collection(db_conn: &DatabaseConn) {
    todo!()
}

fn add_game(db_conn: &DatabaseConn) {
    todo!()
}

fn add_games(db_conn: &DatabaseConn) {
    todo!()
}

fn get_game(db_conn: &DatabaseConn) {
    todo!()
}

fn get_games(db_conn: &DatabaseConn) {
    todo!()
}

fn num_games(db_conn: &DatabaseConn) {
    todo!()
}

fn modify_game(db_conn: &DatabaseConn) {
    todo!()
}

fn modify_games(db_conn: &DatabaseConn) {
    todo!()
}

fn remove_game(db_conn: &DatabaseConn) {
    todo!()
}

fn remove_games(db_conn: &DatabaseConn) {
    todo!()
}

fn drop_all(db_conn: &DatabaseConn) {
    todo!()
}
