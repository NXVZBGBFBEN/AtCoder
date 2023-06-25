use std::vec::Vec;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [[Chars]; n],
    }
    let mut long_s: Vec<char> = Vec::new();
    for c in (0..n).combinations(2) {
        long_s = [s[c[0]], s[c[1]]].concat();
    }
}
