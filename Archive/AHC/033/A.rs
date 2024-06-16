use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        a: [[u32; 5]; 5]
    }
    
    let mut ans: Vec<Vec<char>> = Vec::new();

    for i in 0..5 {
        let mut operation: Vec<char> = Vec::new();

        if i != 0 {
            for _ in 0..ans[i - 1].len() {
                operation.push('.');
            }
        }

        for j in 0..5 {
            let exit_num: isize;

            match a[i][j] {
                0..=4 => exit_num = 0 - i as isize,
                5..=9 => exit_num = 1 - i as isize,
                10..=14 => exit_num = 2 - i as isize,
                15..=19 => exit_num = 3 - i as isize,
                20..=24 => exit_num = 4 - i as isize,
                _ => panic!(),
            }

            operation.push('P');
            for _ in 0..=3 {
                operation.push('R');
            }
            if exit_num < 0 {
               for _ in 0..exit_num.abs() {
                   operation.push('U');
               }
            } else if exit_num > 0 {
               for _ in 0..exit_num.abs() {
                   operation.push('D');
               }
            }
            operation.push('Q');
            if j != 4 {
                if exit_num < 0 {
                   for _ in 0..exit_num.abs() {
                       operation.push('D');
                   }
                } else if exit_num > 0 {
                   for _ in 0..exit_num.abs() {
                       operation.push('U');
                   }
                }
                if j != 4 {
                    for _ in 0..=3 {
                        operation.push('L');
                    }
                }
            } else if i != 4 {
                operation.push('B');
            }
        }

        ans.push(operation);
    }

    print_ans(ans);
}

fn print_ans(ans: Vec<Vec<char>>) {
    for i in ans.iter() {
        for j in i.iter() {
            print!("{j}");
        }
        println!();
    }
}
