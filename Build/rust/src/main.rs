use num_integer::Roots;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{}", n.cbrt())
}
