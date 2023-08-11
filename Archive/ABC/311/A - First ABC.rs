use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: u8,
        s: Chars,
    }
    let mut a = false;
    let mut b = false;
    let mut c = false;
    for (i, j) in s.iter().enumerate() {
        if *j == 'A' { a = true }
        if *j == 'B' { b = true }
        if *j == 'C' { c = true }
        if a && b && c {
            println!("{}", i + 1);
            break;
        }
    }
}
