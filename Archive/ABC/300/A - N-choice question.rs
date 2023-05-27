use proconio::input;

fn main() {
    input! {
        x: [i32; 3],
        y: [i32; x[0]],
    }
    let ans = x[1] + x[2];
    for (i, j) in y.iter().enumerate() {
        if ans == *j {
            println!("{}", i + 1);
        }
    }
}
