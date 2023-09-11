use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c: [[char; 4]; 4],
    }
    for i in (0..4).rev() {
        for j in (0..4).rev() {
            print!("{}", c[i][j]);
            if j != 0 {
                print!(" ")
            }
        }
        print!("\n");
    }
    print!("\n");
}
