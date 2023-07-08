use std::string::String;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut players: [(String, i32); n],
    }
    let youngest_index = players.iter().position(|x| x == players.iter().min_by_key(|p| p.1).unwrap()).unwrap();
    for i in youngest_index..youngest_index + n {
        println!("{}", players.get(i % n).unwrap().0);
    }
}
