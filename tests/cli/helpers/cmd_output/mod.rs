use std::fmt::Display;

pub(crate) use list_tests::*;

mod list_tests;

fn green<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[92m{}\u{1b}[0m", string)
}

fn yellow<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[93m{}\u{1b}[0m", string)
}
