use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars,
        t: Chars
    }
    for (a, b) in s.iter().zip(t.iter()) {
        if a == b || (((a == &'1' && b == &'l') || (a == &'l' && b == &'1')) || ((a == &'0' && b == &'o') || (a == &'o' && b == &'0'))) {
            continue;
        }
        println!("No");
        return;
    }
    println!("Yes");
}
