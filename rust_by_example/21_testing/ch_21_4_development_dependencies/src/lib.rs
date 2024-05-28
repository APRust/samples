// Sometimes there is a need to have dependencies for tests (or examples, or benchmarks) only.
// Such dependencies are added to Cargo.toml in the [dev-dependencies] section.
// These dependencies are not propagated to other packages which depend on this package.
//
// One such example is pretty_assertions, which extends standard assert_eq! and assert_ne! macros,
// to provide colorful diff.
// File Cargo.toml:
//
// # standard crate data is left out
// [dev-dependencies]
// pretty_assertions = "1"
//
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }
}
