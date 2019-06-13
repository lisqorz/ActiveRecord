use std::collections::HashMap;
use crate::ARResult;

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
    pub fn query_scalar(&mut self) -> ARResult<i32> {
        self.query_internal(CommandType::FetchColumn)
    }
    pub fn query_one(&mut self) -> ARResult<i32> {
        self.query_internal(CommandType::FetchOne)
    }
    pub fn query_all(&mut self) -> ARResult<i32> {
        self.query_internal(CommandType::FetchAll)
    }

    fn query_internal(&mut self, method: CommandType) -> ARResult<i32> {
    }

    fn internal_execute(&mut self, stmt: String) -> ARResult<i32> {

    }

    fn prepare(&mut self) {}
    pub fn execute(&mut self) -> ARResult<i32> {
        self.internal_execute("x".parse().unwrap());
    }
}

