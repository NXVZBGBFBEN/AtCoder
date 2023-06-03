use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in 3..=9 {
        if n < 10usize.pow(3) {
            println!("{}", n);
            break;
        } else if 10usize.pow(i) <= n && n < 10usize.pow(i + 1) {
            println!("{}", n / 10usize.pow(i - 2) * 10usize.pow(i - 2));
        }
    }
}
