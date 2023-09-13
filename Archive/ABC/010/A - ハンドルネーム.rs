use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("{}", format!("{}{}", s, "pp"));
}
