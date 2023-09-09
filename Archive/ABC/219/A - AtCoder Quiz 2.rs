use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n < 40 {
        println!("{}", 40 - n);
    } else if n < 70 {
        println!("{}", 70 - n);
    } else if n < 90 {
        println!("{}", 90 - n);
    } else if 90 <= n {
        println!("expert");
    }
}
