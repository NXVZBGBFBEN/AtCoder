use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;
use std::iter::FromIterator;

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n]
    }
    let mut ans = Vec::<Vec<usize>>::new();

    for i in 0..n {
        let mut inner_vec = Vec::<usize>::new();
        for j in 0..n {
            if a[i][j] == 1 {
                inner_vec.push(j + 1);
            }
        }
        ans.push(inner_vec);
    }

    for i in 0..n {
        if ans[i].len() == 0 {
            println!();
        } else {
            let mut ans_iter = ans[i].iter();
            print!("{}", ans_iter.next().unwrap());
            for ans_value in ans_iter {
                print!(" {ans_value}");
            }
            println!();
        }
    }
}
