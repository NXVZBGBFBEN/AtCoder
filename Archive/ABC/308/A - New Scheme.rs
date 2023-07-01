use std::iter::Iterator;
use std::vec::Vec;
use proconio::input;

fn main() {
    input! {
        s: [i32; 8],
    }
    if s.iter().all(|x| *x % 25 == 0 && 100 <= *x && *x <= 675) && s.windows(2).all(|x| x[0] <= x[1]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
