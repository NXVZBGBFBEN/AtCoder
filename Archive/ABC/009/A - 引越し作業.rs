use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u16,
    }
    if n % 2 == 0 {
        println!("{}", n / 2);
    } else {
        println!("{}", n / 2 + 1);
    }
}
