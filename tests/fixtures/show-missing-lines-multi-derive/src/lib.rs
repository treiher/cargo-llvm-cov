use strum::{Display, EnumIter};

#[derive(Display, EnumIter, Clone, Debug, PartialEq)]
pub enum Foo {
    A = 0,
    B = 1,
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_foo_serde() {
        for foo in Foo::iter() {
            assert_eq!(foo, foo.clone());
        }
    }
}
