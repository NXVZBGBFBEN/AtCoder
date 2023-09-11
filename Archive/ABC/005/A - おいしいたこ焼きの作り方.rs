use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (u8, u8),
    }
    println!("{}", y / x)
}
