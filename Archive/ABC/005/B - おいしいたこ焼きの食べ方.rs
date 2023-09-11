use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
        t: [[u8; 1]; n],
    }
    println!("{}", t.iter().flatten().min().unwrap());
}
