extern crate regex;
use regex::Regex;
use std::ops::Range;
use std::slice;

static input: (i32, i32) = (134564, 585159);

pub fn star1() {
    let consecutive_digits =
        Regex::new(r"0{2}|1{2}|2{2}|3{2}|4{2}|5{2}|6{2}|7{2}|8{2}|9{2}").unwrap();
    let result = (input.0..input.1)
        .map(|x| x.to_string())
        .filter(|s| s.bytes().is_sorted())
        .filter(|s| consecutive_digits.is_match(s))
        .count();
    println!("Star 1 {}", result);
}

pub fn star2() {
    let consecutive_digits =
        Regex::new(r"0{2,}|1{2,}|2{2,}|3{2,}|4{2,}|5{2,}|6{2,}|7{2,}|8{2,}|9{2,}").unwrap();
    let result = (input.0..input.1)
        .map(|x| x.to_string())
        .filter(|s| s.bytes().is_sorted())
        .filter(|s| {
            consecutive_digits
                .find_iter(s)
                .any(|m| (m.end() - m.start()) == 2)
        })
        .count();
    println!("Star 2 {}", result);
}
