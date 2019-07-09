use std::collections::HashMap;
use crate::ARResult;
use crate::r2d2_mysql::MysqlConnectionManager;
use crate::r2d2_mysql::r2d2::PooledConnection;

pub enum CommandType {
    FetchAll,
    FetchOne,
    FetchColumn,
}

#[derive(Default)]
pub struct Command {
    params: Vec<String>,
    sql: String,
}

impl Command {
    pub fn new() -> Command {
        Command::default()
    }

    pub fn sql(sql: &str) -> Self {
        let mut a = Self::new();
        a.sql = sql.to_string();
        a
    }
    pub fn query_scalar(&mut self, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        self.query_internal(CommandType::FetchColumn,conn)
    }
    pub fn query_one(&mut self, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        self.query_internal(CommandType::FetchOne, conn)
    }
    pub fn query_all(&mut self, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        self.query_internal(CommandType::FetchAll, conn)
    }

    fn query_internal(&mut self, method: CommandType, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        Ok(1)
    }

    fn internal_execute(&mut self, stmt: String, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        Ok(123)
    }

    fn prepare(&mut self) {}
    pub fn execute(&mut self, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        self.internal_execute("x".parse().unwrap(), conn)
    }
}

