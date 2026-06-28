//! Behavioral conformance suite.
//!
//! The table mirrors the documented behavior one row per case. The first block
//! holds the eleven core assertions. The second block holds the seven usage
//! examples from the readme. The third block holds edge cases derived from the
//! algorithm, each value computed from the reference rule.

use has_flag::has_flag;

fn v(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

struct Case {
    flag: &'static str,
    argv: &'static [&'static str],
    want: bool,
    note: &'static str,
}

/// The argv from the readme usage block: `node foo.js -f --unicorn --foo=bar -- --rainbow`.
const README_ARGV: &[&str] = &[
    "node",
    "foo.js",
    "-f",
    "--unicorn",
    "--foo=bar",
    "--",
    "--rainbow",
];

const CASES: &[Case] = &[
    // --- core assertions (11) ---
    Case {
        flag: "unicorn",
        argv: &["--foo", "--unicorn", "--bar"],
        want: true,
        note: "long flag, auto -- prefix",
    },
    Case {
        flag: "--unicorn",
        argv: &["--foo", "--unicorn", "--bar"],
        want: true,
        note: "optional prefix",
    },
    Case {
        flag: "unicorn=rainbow",
        argv: &["--foo", "--unicorn=rainbow", "--bar"],
        want: true,
        note: "key=value whole element",
    },
    Case {
        flag: "unicorn",
        argv: &["--unicorn", "--", "--foo"],
        want: true,
        note: "before terminator",
    },
    Case {
        flag: "unicorn",
        argv: &["--foo", "--", "--unicorn"],
        want: false,
        note: "after terminator",
    },
    Case {
        flag: "unicorn",
        argv: &["--foo"],
        want: false,
        note: "absent",
    },
    Case {
        flag: "-u",
        argv: &["-f", "-u", "-b"],
        want: true,
        note: "explicit short prefix",
    },
    Case {
        flag: "-u",
        argv: &["-u", "--", "-f"],
        want: true,
        note: "short before terminator",
    },
    Case {
        flag: "u",
        argv: &["-f", "-u", "-b"],
        want: true,
        note: "single char auto - prefix",
    },
    Case {
        flag: "u",
        argv: &["-u", "--", "-f"],
        want: true,
        note: "single char before terminator",
    },
    Case {
        flag: "f",
        argv: &["-u", "--", "-f"],
        want: false,
        note: "single char only after terminator",
    },
    // --- readme usage examples (all over README_ARGV) ---
    Case {
        flag: "unicorn",
        argv: README_ARGV,
        want: true,
        note: "readme: unicorn",
    },
    Case {
        flag: "--unicorn",
        argv: README_ARGV,
        want: true,
        note: "readme: --unicorn",
    },
    Case {
        flag: "f",
        argv: README_ARGV,
        want: true,
        note: "readme: f",
    },
    Case {
        flag: "-f",
        argv: README_ARGV,
        want: true,
        note: "readme: -f",
    },
    Case {
        flag: "foo=bar",
        argv: README_ARGV,
        want: true,
        note: "readme: foo=bar",
    },
    Case {
        flag: "foo",
        argv: README_ARGV,
        want: false,
        note: "readme: foo (no bare --foo)",
    },
    Case {
        flag: "rainbow",
        argv: README_ARGV,
        want: false,
        note: "readme: rainbow after terminator",
    },
    // --- algorithm-derived edge cases ---
    Case {
        flag: "unicorn",
        argv: &[],
        want: false,
        note: "A: empty argv",
    },
    Case {
        flag: "unicorn",
        argv: &["--foo", "--", "--bar"],
        want: false,
        note: "B: absent with terminator present",
    },
    Case {
        flag: "unicorn",
        argv: &["--unicorn", "--unicorn"],
        want: true,
        note: "C: duplicate matches, first index wins",
    },
    Case {
        flag: "unicorn",
        argv: &["--bar"],
        want: false,
        note: "D: absent, no terminator",
    },
    Case {
        flag: "--",
        argv: &["--foo", "--", "--bar"],
        want: false,
        note: "E: flag is -- itself, self-collision",
    },
    Case {
        flag: "",
        argv: &["--", "x"],
        want: false,
        note: "F: empty flag never matches",
    },
    Case {
        flag: "-",
        argv: &["-", "a"],
        want: true,
        note: "G: lone dash flag",
    },
    Case {
        flag: "u",
        argv: &["--", "-u"],
        want: false,
        note: "H: single char after terminator",
    },
    Case {
        flag: "café",
        argv: &["--café"],
        want: true,
        note: "I: multibyte long flag",
    },
    Case {
        flag: "é",
        argv: &["-é"],
        want: true,
        note: "J: precomposed single-scalar short flag",
    },
];

#[test]
fn behavior_parity() {
    for c in CASES {
        let got = has_flag(c.flag, &v(c.argv));
        assert_eq!(
            got, c.want,
            "case `{}`: has_flag({:?}, {:?})",
            c.note, c.flag, c.argv
        );
    }
}

#[test]
fn case_sensitive_match() {
    // Exact case. An uppercase flag does not match a lowercase token.
    assert!(!has_flag("Unicorn", &v(&["--unicorn"])));
    assert!(has_flag("unicorn", &v(&["--unicorn"])));
}

#[test]
fn no_substring_match() {
    // A flag that is a prefix of a token does not match. Whole-token only.
    assert!(!has_flag("foo", &v(&["--foobar"])));
}

#[test]
fn emoji_takes_double_dash_prefix() {
    // U+1F600 is two UTF-16 code units, so it gets the -- prefix, not -.
    assert!(has_flag("\u{1F600}", &v(&["--\u{1F600}"])));
    assert!(!has_flag("\u{1F600}", &v(&["-\u{1F600}"])));
}

#[test]
fn accepts_str_slice_argv() {
    // The generic argv accepts &[&str] as well as &[String].
    let argv: &[&str] = &["--foo", "--unicorn"];
    assert!(has_flag("unicorn", argv));
}
