//! API shape and default-argv glue.
//!
//! These pin the two call shapes (explicit argv and default argv) and exercise
//! the real `has_flag_argv` path against the test binary's own arguments.

use has_flag::{has_flag, has_flag_argv};

#[test]
fn returns_bool_both_arities() {
    let _: bool = has_flag_argv("unicorn");
    let _: bool = has_flag("unicorn", &["--foo".to_string()]);
}

#[test]
fn default_argv_absent_flag() {
    // A flag this odd is not in the test binary's arguments.
    assert!(!has_flag_argv("--definitely-not-a-real-flag-xyz"));
}
