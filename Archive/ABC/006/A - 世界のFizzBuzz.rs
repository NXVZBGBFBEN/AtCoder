use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
    }
    if n % 3 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
