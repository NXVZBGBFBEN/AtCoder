use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let x = 1.0 / n as f64;
    let mut ans = 0.0;
    for i in 1..=n {
        ans += (i * 10000) as f64 * x;
    }
    println!("{}", ans);
}
