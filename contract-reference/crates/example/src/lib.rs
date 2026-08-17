pub use example_api::*;

#[cfg(test)]
mod tests {
    use super::domain_name;

    #[test]
    fn reexports_the_internal_api() {
        assert_eq!(domain_name(), "example");
    }
}
