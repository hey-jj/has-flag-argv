//! Behavioral conformance suite.
//!
//! The table mirrors the documented behavior one row per case. The first block
//! holds the eleven core assertions. The second block holds the seven usage
//! examples from the readme. The third block holds edge cases derived from the
//! algorithm, each value computed from the reference rule.

use has_flag_argv::has_flag;

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
    Case {
        flag: "foo bar",
        argv: &["--foo bar", "--baz"],
        want: true,
        note: "K: flag with an internal space matches as a whole token",
    },
    Case {
        flag: "foo bar",
        argv: &["--foobar"],
        want: false,
        note: "L: space-containing flag is not split or trimmed",
    },
    Case {
        flag: "u",
        argv: &["-u", "--", "-u"],
        want: true,
        note: "M: needle on both sides of terminator, first occurrence wins",
    },
    Case {
        flag: "x",
        argv: &["--", "--x", "--"],
        want: false,
        note: "N: needle only after first terminator, even with a later token",
    },
    Case {
        flag: "--",
        argv: &["--"],
        want: false,
        note: "O: flag is -- in a single-element argv, position equals terminator",
    },
    Case {
        flag: " u",
        argv: &["-u"],
        want: false,
        note: "P: leading space makes the needle `-- u`, which is absent",
    },
    Case {
        flag: "u",
        argv: &[" -u"],
        want: false,
        note: "Q: padded argv token does not equal the clean needle `-u`",
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
fn decomposed_grapheme_takes_double_dash_prefix() {
    // "e" + combining acute (U+0065 U+0301) renders as one glyph but is two
    // UTF-16 code units. The prefix rule counts code units, so this gets --.
    // A char-count rule would wrongly see length 2 here as well, but the real
    // trap is a grapheme-count rule, which would see length 1 and pick a single
    // dash. Pin both directions.
    let decomposed = "e\u{0301}";
    assert!(has_flag(decomposed, &v(&["--e\u{0301}"])));
    assert!(!has_flag(decomposed, &v(&["-e\u{0301}"])));
}

#[test]
fn accepts_str_slice_argv() {
    // The generic argv accepts &[&str] as well as &[String].
    let argv: &[&str] = &["--foo", "--unicorn"];
    assert!(has_flag("unicorn", argv));
}

#[test]
fn explicit_prefix_agrees_with_auto() {
    // Naming a flag with its dash prefix gives the same result as letting the
    // function pick the prefix. Check both the long and short forms across a
    // mixed argv.
    let argv: &[&str] = &["--foo", "-u", "--unicorn", "--", "--bar"];
    assert_eq!(has_flag("unicorn", argv), has_flag("--unicorn", argv));
    assert_eq!(has_flag("u", argv), has_flag("-u", argv));
    assert_eq!(has_flag("bar", argv), has_flag("--bar", argv));
}

#[test]
fn never_panics_on_edge_inputs() {
    // The function takes `&str`, so input is always valid UTF-8 and cannot
    // panic. Pin that with dash edges, multibyte scalars, internal `--`, and an
    // empty flag against an argv built from the same shapes.
    let argv: &[&str] = &["", "-", "--", "---", "-x", "--x", "café", "😀", "a b"];
    let flags = ["", "-", "--", "---", "x", "-x", "--x", "café", "😀", "a b"];
    for flag in flags {
        // The only assertion is that the call returns without panicking.
        let _ = has_flag(flag, argv);
    }
}
