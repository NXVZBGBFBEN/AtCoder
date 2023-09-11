use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut w: Chars,
    }
    w.retain(|x| x != &'a' && x != &'i' && x != &'u' && x != &'e' && x != &'o');
    for i in w.iter() {
        print!("{}", i);
    }
    print!("\n");
}
