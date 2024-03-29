use std::error::Error;
use crate::query_builder::QueryBuilderTrait;
use super::ARResult;

use crate::r2d2_mysql::MysqlConnectionManager;
use crate::r2d2_mysql::r2d2::PooledConnection;
use crate::query::Query;
use crate::command::Command;

// todo 考虑把trait和struct合并一起用
pub trait ActiveRecordTrait {

}

pub struct ActiveRecord {}


impl QueryBuilderTrait for ActiveRecord {}

impl ActiveRecord {
    fn alias(&mut self, name: &AsRef<str>) -> &mut Self {
        self
    }
    fn left_join(&mut self) -> &mut Self {
        self
    }
    fn right_join(&mut self) -> &mut Self {
        self
    }
    fn inner_join(&mut self) -> &mut Self {
        self
    }
    fn r#where(&mut self) -> &mut Self {
        self
    }
    fn and_where(&mut self) -> &mut Self { self }

    fn filter_where(&mut self) -> &mut Self {
        self
    }
    fn and_filter_where(&mut self) -> &mut Self { self }

    fn select(&mut self) -> &mut Self { self }

    fn group_by(&mut self) -> &mut Self { self }

    fn limit(&mut self) -> &mut Self { self }
    fn find() -> ActiveRecord {
        ActiveRecord {}
    }
    fn find_by_sql() {}


    fn find_one() {}
    fn update_all() {}
    fn update_all_counter() {}
    fn delete_all() {}
    fn is_new_record() -> bool { true }
    fn find_by_condition(condition: String) {
        let primary_key = "id";
    }

    pub fn one(&mut self, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<i32> {
        Command::sql("select * from sys_config").query_one(conn);
        Ok(1)
    }

    pub fn all(&mut self, conn: &PooledConnection<MysqlConnectionManager>) -> ARResult<Vec<i32>> {
        let res = Vec::new();
        Ok(res)
    }

    pub fn batch(&mut self, size: i32) -> ARResult<Vec<i32>> {
        let res = Vec::new();
        Ok(res)
    }
    pub fn count(&mut self) -> ARResult<i32> {
        Ok(1)
    }

    pub fn exists(&mut self) -> ARResult<bool> {
        Ok(true)
    }

    pub fn scalar(&mut self) -> ARResult<i32> {
        Ok(1)
    }

    pub fn sum<T: AsRef<str>>(&mut self, field: T) -> ARResult<i32> {
        Ok(1)
    }
}
