//! Helper binary for the default-argv path.
//!
//! It prints whether `target` is present in its own process arguments. The
//! integration tests run it with crafted args to prove [`has_flag_argv`] reads
//! the real argument vector.
//!
//! [`has_flag_argv`]: has_flag::has_flag_argv

fn main() {
    println!("{}", has_flag::has_flag_argv("target"));
}
