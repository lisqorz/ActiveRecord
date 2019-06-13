use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub trait QueryBuilderTrait {
    fn alias(&mut self, name: &AsRef<str>) {}
    fn left_join(&mut self) {}
    fn right_join(&mut self) {}

    fn inner_join(&mut self) {}
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
}

