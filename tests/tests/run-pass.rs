// Copyright 2016 Indoc Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "unstable", feature(proc_macro_hygiene))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate indoc;
use indoc::indoc;

#[test]
fn byte_string() {
    let indoc = indoc!(b"
        a

            \\b
        c");
    let expected = b"a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn carriage_return() {
    // Every line in the string ends with \r\n
    let indoc = indoc!("
        a

            \\b
        c");
    let expected = "a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn empty_string() {
    let indoc = indoc!("");
    let expected = "";
    assert_eq!(indoc, expected);
}

#[test]
fn joined_first_line() {
    let indoc = indoc!("\
        a");
    let expected = "a";
    assert_eq!(indoc, expected);
}

#[test]
fn joined_lines() {
    let indoc = indoc!("
        a\
        b
        c\
          d
        e");
    let expected = "ab\ncd\ne";
    assert_eq!(indoc, expected);
}

#[test]
fn no_leading_newline() {
    let indoc = indoc!("a
                        b
                        c");
    let expected = "a\nb\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn one_line() {
    let indoc = indoc!("a");
    let expected = "a";
    assert_eq!(indoc, expected);
}

#[test]
fn raw_byte_string() {
    let indoc = indoc!(br#"
        "a"

            \\b
        c"#);
    let expected = b"\"a\"\n\n    \\\\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn raw_string() {
    let indoc = indoc!(r#"
        "a"

            \\b
        c"#);
    let expected = "\"a\"\n\n    \\\\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn string() {
    let indoc = indoc!("
        a

            \\b
        c");
    let expected = "a\n\n    \\b\nc";
    assert_eq!(indoc, expected);
}

#[test]
fn string_trailing_newline() {
    let indoc = indoc!("
        a

            \\b
        c
    ");
    let expected = "a\n\n    \\b\nc\n";
    assert_eq!(indoc, expected);
}

#[test]
fn trailing_whitespace() {
    let indoc = indoc!("
        2 below
          
        0 below
        
        -2 below
      
        end");
    let expected = "2 below\n  \n0 below\n\n-2 below\n\nend";
    assert_eq!(indoc, expected);
}

#[cfg(feature = "unstable")]
#[test]
fn indoc_as_format_string() {
    let s = format!(indoc!("{}"), true);
    assert_eq!(s, "true");
}
