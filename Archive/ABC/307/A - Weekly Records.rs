use std::iter::{IntoIterator, Iterator};
use std::string::String;
use std::vec::Vec;
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [i32; 7 * n],
    }
    let mut sum: Vec<String> = Vec::new();
    for i in &a.into_iter().chunks(7) {
        sum.push(i.sum::<i32>().to_string())
    }
    println!("{}", sum.join(" "));
}
