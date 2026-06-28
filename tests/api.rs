//! API shape and default-argv glue.
//!
//! These pin the two call shapes (explicit argv and default argv) and prove the
//! default path reads the process arguments end to end.

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

#[test]
fn default_argv_reads_process_args() {
    // Run the probe binary with a crafted argument vector and confirm the
    // default path finds the flag.
    let present = std::process::Command::new(env!("CARGO_BIN_EXE_probe"))
        .args(["--before", "--target", "--after"])
        .output()
        .expect("probe binary runs");
    assert_eq!(String::from_utf8_lossy(&present.stdout).trim(), "true");

    // Same probe, flag placed after the terminator, must report false.
    let after_terminator = std::process::Command::new(env!("CARGO_BIN_EXE_probe"))
        .args(["--before", "--", "--target"])
        .output()
        .expect("probe binary runs");
    assert_eq!(
        String::from_utf8_lossy(&after_terminator.stdout).trim(),
        "false"
    );

    // Flag absent entirely.
    let absent = std::process::Command::new(env!("CARGO_BIN_EXE_probe"))
        .args(["--before", "--after"])
        .output()
        .expect("probe binary runs");
    assert_eq!(String::from_utf8_lossy(&absent.stdout).trim(), "false");
}
