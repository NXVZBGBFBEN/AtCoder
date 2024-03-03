use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        (a, b): (i32, i32)
    }
    for i in 0..10 {
        if a + b != i {
            println!("{i}");
            break;
        }
    }
}
