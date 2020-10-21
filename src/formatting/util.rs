use crate::config::*;
use std::cmp::Ordering;
use std::fmt::Write;

pub fn longest_hint<'a>(hint1: &'a str, hint2: &'a str) -> &'a str {
    return match hint1.len().cmp(&hint2.len()) {
        Ordering::Less => hint2,
        Ordering::Greater => hint1,
        Ordering::Equal => hint1,
    };
}

pub fn trim_end_spaces_and_tabs<'a>(string: &'a String) -> &'a str {
    string.trim_end_matches(|ch: char| return ch == ' ' || ch == '\t')
}

pub fn get_positon_after_newline(s: &str, _cfg: &Config) -> usize {
    let idx_opt = s.rfind('\n');
    return s.len() - idx_opt.unwrap_or(0);
}

pub fn write_indent(f: &mut String, cfg: &Config, state: &State) -> std::fmt::Result {
    let indentation = match &cfg.indentation_string {
        Some(indent_str) => (0..state.indent_level).map(|_| &indent_str[..]).collect::<String>(),
        None => String::new(),
    };

    write!(f, "{}", indentation)
}

#[test]
fn test_trim_end_spaces_and_tabs() {
    assert_eq!(trim_end_spaces_and_tabs(&"abc\t  \t  ".to_string()), "abc");
    assert_eq!(trim_end_spaces_and_tabs(&"abc\t  \n\t  ".to_string()), "abc\t  \n");
    assert_eq!(trim_end_spaces_and_tabs(&"abc\t  \r\t  ".to_string()), "abc\t  \r");
}

#[test]
fn test_position_after_newline() {
    let cfg = Config::default();
    assert_eq!(get_positon_after_newline("abc\t  \n  ", &cfg), 3);
    assert_eq!(get_positon_after_newline("abc\t  \n  absdsrf", &cfg), 10);
    assert_eq!(get_positon_after_newline("\nabc\t dasdsadas \n  asdasdas\nabsdsrf", &cfg), 8);
}
