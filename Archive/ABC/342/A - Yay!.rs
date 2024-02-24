use proconio::{fastout, input};
use proconio::marker::Chars;
use std::collections::HashSet;
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let unique: HashSet<char> = s.clone().into_iter().collect();
    let vec_unique = Vec::from_iter(unique);

    let c1_num = s.iter().filter(|&c| *c == vec_unique[0]).count();
    let c2_num = s.iter().filter(|&c| *c == vec_unique[1]).count();

    let c_pos: usize = if c1_num < c2_num {
        s.iter().position(|c| c == &vec_unique[0]).unwrap()
    } else {
        s.iter().position(|c| c == &vec_unique[1]).unwrap()
    };

    println!("{}", c_pos + 1);
}
