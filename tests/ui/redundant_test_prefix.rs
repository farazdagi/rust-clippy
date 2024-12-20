#![allow(dead_code)]
#![warn(clippy::redundant_test_prefix)]

fn main() {
    // Normal function, no redundant prefix.
}

fn f1() {
    // Normal function, no redundant prefix.
}

fn test_f2() {
    // Has prefix, but no `#[test]` attribute, ignore.
}

#[test]
fn test_f3() {
    // Has prefix, has `#[test]` attribute.
    // Not within a `#[cfg(test)]`, ignore (by default, configurable -- see TOML tests).
}

#[cfg(test)]
#[test]
fn test_f4() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix
    //~| NOTE: `-D clippy::redundant-test-prefix` implied by `-D warnings`

    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // No collision with other functions, should `test_` prefix be removed.
}

fn f5() {}

#[cfg(test)]
#[test]
fn test_f5() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix (suffix avoids name conflict)

    todo!()
    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // Collision with existing function, so suffix `_works` is added.
}

mod m1 {
    pub fn f6() {}
    pub fn f7() {}
}

#[cfg(test)]
#[test]
fn test_f6() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix (suffix avoids name conflict)

    use m1::f6;

    f6();
    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // No collision, but has a function call that will result in recursion.
}

#[cfg(test)]
#[test]
fn test_f8() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix

    use m1::f7;

    f7();
    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // No collision, has function call, but it will not result in recursion.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix (suffix avoids name conflict)

        main();
    }

    #[test]
    fn test_foo() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_foo_with_call() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix

        main();
    }

    #[test]
    fn test_f1() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f2() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f3() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f4() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f5() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f6() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f7() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }

    #[test]
    fn test_f8() {
        //~^ ERROR: redundant `test_` prefix in test function name
        //~| HELP: consider removing the `test_` prefix
    }
}

mod tests_no_annotations {
    use super::*;

    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_foo() {}

    #[test]
    fn test_foo_with_call() {
        main();
    }

    #[test]
    fn test_f1() {}

    #[test]
    fn test_f2() {}

    #[test]
    fn test_f3() {}

    #[test]
    fn test_f4() {}

    #[test]
    fn test_f5() {}

    #[test]
    fn test_f6() {}

    #[test]
    fn test_f7() {}

    #[test]
    fn test_f8() {}
}

// This test is inspired by real test in `clippy_utils/src/sugg.rs`.
// The `is_in_test_function()` checks whether any identifier within a given node's parents is
// marked with `#[test]` attribute. Thus flagging false positives when nested functions are
// prefixed with `test_`. Therefore `is_test_function()` has been defined in `clippy_utils`,
// allowing to select only functions that are immediately marked with `#[test]` annotation.
//
// This test case ensures that for such nested functions no error is emitted.
#[test]
fn not_op() {
    fn test_not(foo: bool) {
        assert!(foo);
    }

    // Use helper function
    test_not(true);
    test_not(false);
}
