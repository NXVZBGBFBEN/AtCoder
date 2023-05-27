use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: u8,
        eval: Chars
    }
    let cond_a = eval.iter().any(|&s| s == 'o');
    let cond_b = !eval.iter().any(|&s| s == 'x');
    if cond_a && cond_b {
        println!("Yes");
    } else {
        println!("No");
    }
}
