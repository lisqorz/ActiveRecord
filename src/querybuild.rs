use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub trait QueryBuildTrait{
    fn alias(&mut self, name: &AsRef<str>) {}
    fn left_join(&mut self) {}
    fn right_join(&mut self) {}

    fn inner_join(&mut self) {}
    fn and_where(&mut self) -> &mut Self { self }

    fn and_filter(&mut self) -> &mut Self { self }

    fn select(&mut self) -> &mut Self { self }

    fn group_by(&mut self) -> &mut Self { self }

    fn limit(&mut self) -> &mut Self { self }

}
impl QueryBuildTrait for QueryBuilder{}
struct QueryBuilder {}

impl Debug for QueryBuilder {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"")
    }
}

impl Display for QueryBuilder {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"")
    }
}


