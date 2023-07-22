use std::vec::Vec;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n_d: (usize, usize),
        s: [Chars; n_d.0]
    }
    let mut all_o = true;
    let mut result: Vec<bool> = Vec::new();
    for i in 0..n_d.1 {
        for j in 0..n_d.0 {
            if s[j][i] == 'x' { all_o = false; }
        }
        if all_o {
            result.push(true);
        } else {
            result.push(false);
            all_o = true;
        }
    }
    let mut max = 0;
    let mut current = 0;
    for i in result.iter() {
        if *i {
            current += 1;
        } else {
            current = 0;
        }
        if max < current {
            max = current;
        }
    }
    println!("{}", max);
}
