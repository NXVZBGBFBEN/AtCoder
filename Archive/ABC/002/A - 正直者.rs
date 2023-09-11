use proconio::input;

fn main() {
    input! {
        nums: [isize; 2],
    }
    println!("{}", nums.iter().max().unwrap());
}
