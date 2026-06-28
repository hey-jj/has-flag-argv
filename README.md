# has-flag-rs

Detect whether a single command-line flag is present in an argv-style list.

This is a presence check. It returns a boolean. The dash prefix is optional.
A single-character flag gets one dash, a longer flag gets two. The search stops
at the POSIX `--` terminator, so a flag at or after the first `--` counts as
absent.

## Installation

```toml
[dependencies]
has-flag-rs = "0.1"
```

The library is imported as `has_flag`.

## Usage

```rust
use has_flag::has_flag;

let argv = ["--foo", "--unicorn", "--bar"].map(String::from);

assert!(has_flag("unicorn", &argv));   // auto -- prefix
assert!(has_flag("--unicorn", &argv)); // explicit prefix works too
assert!(!has_flag("rainbow", &argv));  // absent
```

Search the process arguments instead of an explicit list:

```rust
use has_flag::has_flag_argv;

if has_flag_argv("verbose") {
    // run with extra logging
}
```

## API

### `has_flag(flag, argv) -> bool`

Check whether `flag` is present in `argv`. The `argv` parameter is any slice of
string-like values (`&[String]` or `&[&str]`). The dash prefix on `flag` is
optional.

Matching is exact, case-sensitive, whole-token equality. No substring match, no
`=` splitting, no trimming, no Unicode normalization.

### `has_flag_argv(flag) -> bool`

Like `has_flag`, but reads the process arguments for the argument list. It
reads the arguments fresh on each call and skips any argument that is not valid
Unicode.

## Behavior notes

- Prefix rule: a flag that already starts with `-` is searched verbatim. A flag
  one UTF-16 code unit long gets a single `-`. Anything longer gets `--`.
- An empty flag never matches. A flag equal to `--` never matches, because the
  searched token would collide with the terminator.
- Only the first occurrence of the flag matters, compared against the first
  `--` terminator.

## License

Licensed under the [MIT license](LICENSE).
