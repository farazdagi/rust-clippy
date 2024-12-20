//@revisions: default outside_cfg_test custom_suffix
//@[default] rustc-env:CLIPPY_CONF_DIR=tests/ui-toml/redundant_test_prefix/default
//@[outside_cfg_test] rustc-env:CLIPPY_CONF_DIR=tests/ui-toml/redundant_test_prefix/outside_cfg_test
//@[custom_suffix] rustc-env:CLIPPY_CONF_DIR=tests/ui-toml/redundant_test_prefix/custom_suffix
//@compile-flags: --test
#![allow(dead_code)]
#![warn(clippy::redundant_test_prefix)]

fn main() {}

mod tests_no_annotations {
    use super::*;

    #[test]
    fn test_has_annotation() {
        //~[outside_cfg_test]^ redundant_test_prefix
    }

    fn no_annotation() {}
}

#[test]
fn test_main_module_has_annotation() {
    //~[outside_cfg_test]^ redundant_test_prefix
}

fn test_main_module_no_annotation() {}

fn foo() {}

#[cfg(test)]
#[test]
fn test_foo() {
    //~^ ERROR: redundant `test_` prefix in test function name
    //~| HELP: consider removing the `test_` prefix (suffix avoids name conflict)

    todo!()
    // Has prefix, has `#[test]` attribute, within a `#[cfg(test)]`.
    // Collision with existing function, so suffix is added.
}

fn bar() {}

#[test]
fn test_bar() {
    //~[custom_suffix]^ redundant_test_prefix

    todo!()
    // Has prefix, has `#[test]` attribute.
    // NOT within a `#[cfg(test)]`, but the lint is enabled for integration tests.
    // Collision with existing function, so suffix is added.
}

#[cfg(test)]
mod tests {}
