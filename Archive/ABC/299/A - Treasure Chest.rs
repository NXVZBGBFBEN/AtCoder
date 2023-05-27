use std::vec::Vec;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _ : usize,
        input: Chars
    }

    let mut pure_input: Vec<char> = Vec::new();

    for i in input {
        match i {
            '|' => pure_input.push('|'),
            '*' => pure_input.push('*'),
            _ => continue,
        }
    }

    match pure_input[0] {
        '|' => match pure_input[1] {
            '*' => match pure_input[2] {
                '|' => println!("in"),
                _ => println!("out")
            }
            _ => println!("out")
        }
        _ => println!("out")
    }
}
