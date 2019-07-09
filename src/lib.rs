mod active_record;
mod active_query;
mod query_builder;
mod query;
mod command;
mod mysql;

extern crate r2d2_mysql;

use std::io::ErrorKind;

type ARResult<T> = Result<T, ErrorKind>;

#[cfg(test)]
mod tests {
    use crate::query_builder::QueryBuilderTrait;
    use std::sync::Arc;
    use std::thread::Builder;

    extern crate r2d2_mysql as r2d2;

    #[test]
    fn scalar() {
        use crate::active_record::ActiveRecordTrait;
        use r2d2_mysql::mysql::OptsBuilder;
        struct User {
            id: i32
        }
        {
            impl ActiveRecordTrait for User {}
        }
        let db_url = "mysql://root:@localhost:3306/sys";
        let builder = OptsBuilder::from_opts(db_url);
        let manager = r2d2_mysql::MysqlConnectionManager::new(builder);
        let b = r2d2_mysql::r2d2::Pool::new(manager);
        let pool = Arc::new(b.unwrap());
        let conn = pool.clone().get().unwrap();

        let a = User::find().r#where().and_where().and_filter_where().and_where().one(&conn).and_then(|x| {});
        let a = User::find().r#where().and_where().filter_where().all(&conn).for_each(|x| {});
    }

    #[test]
    fn query_scalar() {
        use crate::command::Command;
    }
}
