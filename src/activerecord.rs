use std::error::Error;
use std::io::ErrorKind;
use crate::querybuild::QueryBuildTrait;
// todo 考虑把trait和struct合并一起用
pub trait ActiveRecordTrait {
    fn find() -> ActiveRecord {
        ActiveRecord {}
    }
    fn find_one() {}
    fn update_all() {}
    fn update_all_counter() {}
    fn delete_all() {}
    fn is_new_record() -> bool {true}
}

pub struct ActiveRecord {}

type ARResult<T> = Result<T, ErrorKind>;

impl QueryBuildTrait for ActiveRecord{}

impl ActiveRecord {

    pub fn one(&mut self) -> ARResult<i32> {
        Ok(1)
    }

    pub fn all(&mut self) -> ARResult<Vec<i32>> {
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