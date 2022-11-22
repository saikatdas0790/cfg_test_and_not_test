use contains_lib::Foo;

fn uses_foo() {
    Foo::foo();
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{predicate::*, *};

    mock! {
        pub Foo {
            fn foo();
        }
    }

    #[test]
    fn test_uses_foo() {
        MockFoo::foo_context()
            .expect()
            .returning(|| println!("from mock"));

        uses_foo();
    }
}
