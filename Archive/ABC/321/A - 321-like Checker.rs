use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    if n.len() == 1 {
        println!("Yes");
    } else {
        for i in (1..(n.len())).rev() {
            if n[i].to_digit(10).unwrap() < n[i - 1].to_digit(10).unwrap() {
                if i == 1 {
                    println!("Yes");
                } else {
                    continue;
                }
            } else {
                println!("No");
                break;
            }
        }
    }
}
