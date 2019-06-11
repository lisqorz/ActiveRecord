mod activerecord;
mod querybuild;

#[cfg(test)]
mod tests {
    #[test]
    fn scalar() {
        use crate::activerecord::ActiveRecordTrait;
        use crate::querybuild::QueryBuildTrait;
        struct User {
            id: i32
        }
        impl ActiveRecordTrait for User {}
        let a = User::find().and_where().and_where().one();
    }
}
