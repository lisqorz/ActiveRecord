mod active_record;
mod active_query;
mod query_builder;
mod query;
mod command;
mod mysql;
use std::io::ErrorKind;
type ARResult<T> = Result<T, ErrorKind>;

#[cfg(test)]
mod tests {
    use crate::query_builder::QueryBuilderTrait;

    #[test]
    fn scalar() {
        use crate::active_record::ActiveRecordTrait;
        struct User {
            id: i32
        }
        impl ActiveRecordTrait for User {}
        let a = User::find().r#where().and_where().and_filter_where().and_where().one();
    }
    #[test]
    fn query_scalar() {
        use crate::command::Command;
        Command::sql("select * from t1").query_one();
    }
}
