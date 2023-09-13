use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (s, t): (u16, u16),
    }
    println!("{}", t - s + 1);
}
