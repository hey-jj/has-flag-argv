//! Property tests for the prefix and terminator rules.
//!
//! These encode invariants across random input and catch off-by-one or prefix
//! mistakes that fixed cases can miss.

use has_flag::has_flag;
use proptest::prelude::*;

/// Random argv tokens: bare words, short-prefixed, long-prefixed, or `--`.
fn token() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-z]{1,6}",
        "[a-z]{1,6}".prop_map(|s| format!("-{s}")),
        "[a-z]{1,6}".prop_map(|s| format!("--{s}")),
        Just("--".to_string()),
    ]
}

proptest! {
    // A long flag placed first is always found, since any terminator is later.
    #[test]
    fn present_long_flag_before_terminator(
        name in "[a-z]{2,6}",
        noise in prop::collection::vec(token(), 0..5),
    ) {
        let mut argv = noise;
        argv.insert(0, format!("--{name}"));
        prop_assert!(has_flag(&name, &argv));
    }

    // Nothing strictly after the first `--` is matched as a flag.
    #[test]
    fn nothing_after_terminator(
        name in "[a-z]{2,6}",
        head in prop::collection::vec("[a-z]{2,6}".prop_map(|s| format!("--{s}")), 0..4),
    ) {
        prop_assume!(!head.iter().any(|t| t == &format!("--{name}")));
        let mut argv = head;
        argv.push("--".to_string());
        argv.push(format!("--{name}"));
        prop_assert!(!has_flag(&name, &argv));
    }

    // Explicit `--` prefix and the auto prefix agree for a long flag.
    #[test]
    fn explicit_matches_auto_long(
        name in "[a-z]{2,6}",
        argv in prop::collection::vec(token(), 0..6),
    ) {
        prop_assert_eq!(
            has_flag(&name, &argv),
            has_flag(&format!("--{name}"), &argv),
        );
    }

    // Explicit `-` prefix and the auto prefix agree for a single-char flag.
    #[test]
    fn explicit_matches_auto_short(
        c in "[a-z]",
        argv in prop::collection::vec(token(), 0..6),
    ) {
        prop_assert_eq!(
            has_flag(&c, &argv),
            has_flag(&format!("-{c}"), &argv),
        );
    }

    // Never panics on arbitrary input.
    #[test]
    fn never_panics(
        flag in ".*",
        argv in prop::collection::vec(".*", 0..8),
    ) {
        let _ = has_flag(&flag, &argv);
    }
}
