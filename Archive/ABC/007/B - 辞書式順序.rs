use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: Chars,
    }
    if a.len() == 1 && a[0] == 'a' {
        println!("-1");
    } else {
        println!("a");
    }
}
