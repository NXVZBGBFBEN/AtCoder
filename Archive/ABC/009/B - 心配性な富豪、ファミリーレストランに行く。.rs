use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u8,
        a: [[u16; 1]; n],
    }
    let mut a_concat = a.concat();
    a_concat.sort();
    a_concat.dedup();
    a_concat.reverse();
    println!("{:?}", a_concat[1]);
}
