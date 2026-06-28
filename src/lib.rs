//! Detect whether a single command-line flag is present in an `argv`-style
//! list of strings.
//!
//! This is a presence check. It returns a boolean and never reports the flag's
//! value, position, or count. The leading dash prefix is optional. When you
//! omit it, the function picks the conventional prefix from the flag's shape:
//! a single-character flag gets one dash, a longer flag gets two.
//!
//! Search stops at the POSIX `--` terminator. A flag that appears at or after
//! the first standalone `--` token counts as absent.
//!
//! # Examples
//!
//! ```
//! use has_flag::has_flag;
//!
//! let argv = ["--foo", "--unicorn", "--bar"].map(String::from);
//!
//! assert!(has_flag("unicorn", &argv)); // auto `--` prefix
//! assert!(has_flag("--unicorn", &argv)); // explicit prefix also works
//! assert!(!has_flag("rainbow", &argv)); // not present
//! ```
//!
//! Reading from the process arguments:
//!
//! ```
//! use has_flag::has_flag_argv;
//!
//! // Looks through std::env::args() for `--verbose`.
//! let _present: bool = has_flag_argv("verbose");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Check whether `flag` is present in `argv`.
///
/// The dash prefix on `flag` is optional. The rule that picks the prefix:
///
/// - If `flag` already starts with `-`, search for it verbatim.
/// - Otherwise, if `flag` is one UTF-16 code unit long, prepend a single `-`.
/// - Otherwise, prepend `--`.
///
/// Length is counted in UTF-16 code units to match the behavior of the
/// JavaScript library this mirrors. For ASCII flags this equals the character
/// count. A non-BMP scalar such as an emoji counts as two units, so it takes
/// the `--` prefix.
///
/// Matching is exact, case-sensitive, whole-token equality. No substring match,
/// no `=` splitting, no trimming, no Unicode normalization.
///
/// The search honors the POSIX `--` terminator. The function returns `true`
/// only when the flag's first occurrence comes before the first `--` token. A
/// flag found at or after `--` returns `false`.
///
/// Two edge cases follow from this and are worth pinning. When the searched
/// token is itself `--` (an empty flag, or `flag == "--"`), the flag position
/// equals the terminator position, so the strict before-check fails and the
/// result is `false`. An empty flag never matches.
///
/// # Examples
///
/// ```
/// use has_flag::has_flag;
///
/// let argv = ["-f", "-u", "-b"].map(String::from);
/// assert!(has_flag("u", &argv)); // single char -> `-u`
/// assert!(has_flag("-u", &argv)); // explicit `-u`
///
/// let with_terminator = ["--foo", "--", "--unicorn"].map(String::from);
/// assert!(!has_flag("unicorn", &with_terminator)); // after `--`
/// ```
pub fn has_flag<S: AsRef<str>>(flag: &str, argv: &[S]) -> bool {
    let prefix = if flag.starts_with('-') {
        ""
    } else if flag.encode_utf16().count() == 1 {
        "-"
    } else {
        "--"
    };

    let needle = format!("{prefix}{flag}");

    let position = argv.iter().position(|arg| arg.as_ref() == needle);
    let terminator = argv.iter().position(|arg| arg.as_ref() == "--");

    match (position, terminator) {
        (Some(found), Some(stop)) => found < stop,
        (Some(_), None) => true,
        (None, _) => false,
    }
}

/// Check whether `flag` is present in the process arguments.
///
/// This reads `std::env::args()` on each call and forwards to [`has_flag`]. The
/// whole argument vector is searched as-is, including the program path at index
/// zero. A flag needle never realistically matches that path, so the leading
/// element is harmless.
///
/// # Examples
///
/// ```
/// use has_flag::has_flag_argv;
///
/// // Almost certainly absent from a test binary's arguments.
/// assert!(!has_flag_argv("--definitely-not-a-real-flag-xyz"));
/// ```
pub fn has_flag_argv(flag: &str) -> bool {
    let argv: Vec<String> = std::env::args().collect();
    has_flag(flag, &argv)
}
