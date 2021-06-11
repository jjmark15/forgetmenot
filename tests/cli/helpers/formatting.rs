use std::fmt::Display;

pub(crate) fn green<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[92m{}\u{1b}[0m", string)
}

pub(crate) fn yellow<S: AsRef<str> + Display>(string: S) -> String {
    format!("\u{1b}[93m{}\u{1b}[0m", string)
}
