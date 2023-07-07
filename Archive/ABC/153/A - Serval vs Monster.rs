use proconio::input;

fn main() {
    input! {
        mut h_a: [i32; 2],
    }
    let mut cnt = 0;
    while h_a[0] > 0 {
        h_a[0] -= h_a[1];
        cnt += 1;
    }
    println!("{}", cnt);
}
