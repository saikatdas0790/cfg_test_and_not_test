#[cfg(not(test))]
pub fn some_function() {
    panic!("for not test");
}

#[cfg(test)]
pub fn some_function() {
    println!("for_tests");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_function() {
        some_function();
    }
}
