use std::marker::PhantomData;

use mysql::prelude::{FromRow, Queryable};
use mysql::{OptsBuilder, Params, Pool, PooledConn};

pub struct DBConn<T>
where
    T: FromRow,
{
    conn: PooledConn,
    table: String,
    phantom: PhantomData<T>,
}

impl<T> DBConn<T>
where
    T: FromRow,
{
    // Constructor
    pub fn connect(
        user: Option<String>,
        pass: Option<String>,
        ip: Option<String>,
        port: u16,
        database: Option<String>,
        table: String,
    ) -> Result<Self, String> {
        let opts = OptsBuilder::new()
            .user(user)
            .pass(pass)
            .ip_or_hostname(ip)
            .tcp_port(port)
            .db_name(database);

        let pool = match Pool::new(opts) {
            Ok(pool) => pool,
            Err(err) => return Err(err.to_string()),
        };

        let conn = match pool.get_conn() {
            Ok(conn) => conn,
            Err(err) => return Err(err.to_string()),
        };

        Ok(Self {
            conn,
            table,
            phantom: PhantomData,
        })
    }

    pub fn set_table(&mut self, table: String) {
        self.table = table;
    }

    pub fn insert(&mut self, thing: &impl DBInsert) -> Result<(), String> {
        let prep = match self.conn.prep(thing.ins_statement(&self.table)) {
            Ok(statement) => statement,
            Err(err) => return Err(err.to_string()),
        };

        let param = thing.ins_parameter();

        match self.conn.exec_drop(&prep, param) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn modify(&mut self, thing: &impl DBModify) -> Result<(), String> {
        let prep = match self.conn.prep(thing.mod_statement(&self.table)) {
            Ok(statement) => statement,
            Err(err) => return Err(err.to_string()),
        };

        let param = thing.mod_parameter();

        match self.conn.exec_drop(&prep, param) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn query(&mut self, thing: &impl DBQuery) -> Result<Vec<T>, String> {
        let prep = match self.conn.prep(thing.query_statement(&self.table, false)) {
            Ok(statement) => statement,
            Err(err) => return Err(err.to_string()),
        };

        let param = thing.query_parameter();

        let response: Vec<T> = match self.conn.exec(&prep, param) {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        Ok(response)
    }

    pub fn query_sort(&mut self, thing: &impl DBQuery) -> Result<Vec<T>, String> {
        let prep = match self.conn.prep(thing.query_statement(&self.table, true)) {
            Ok(statement) => statement,
            Err(err) => return Err(err.to_string()),
        };

        let param = thing.query_parameter();

        let response: Vec<T> = match self.conn.exec(&prep, param) {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        Ok(response)
    }

    pub fn delete(&mut self, thing: &impl DBDelete) -> Result<(), String> {
        let prep = match self.conn.prep(thing.del_statement(&self.table)) {
            Ok(statement) => statement,
            Err(err) => return Err(err.to_string()),
        };

        let param = thing.del_parameter();

        match self.conn.exec_drop(&prep, param) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn wipe(&mut self) -> Result<(), String> {
        match self.conn.query_drop(format!("DELETE FROM {}", self.table)) {
            Ok(()) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub trait DBInsert {
    fn ins_statement(&self, table: &str) -> String;
    fn ins_parameter(&self) -> Params;
}

pub trait DBModify {
    fn mod_statement(&self, table: &str) -> String;
    fn mod_parameter(&self) -> Params;
}

pub trait DBQuery {
    fn query_statement(&self, table: &str, sort: bool) -> String;
    fn query_parameter(&self) -> Params;
}

pub trait DBDelete {
    fn del_statement(&self, table: &str) -> String;
    fn del_parameter(&self) -> Params;
}
