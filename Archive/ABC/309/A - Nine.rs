use proconio::input;

fn main() {
    input! {
        input: (i32, i32),
    }
    if input.0 % 3 == 0 || (input.1 + 2) % 3 == 0 {
        println!("No")
    } else if 2 <= (input.1 - input.0) {
        println!("No")
    } else {
        println!("Yes")
    }
}
