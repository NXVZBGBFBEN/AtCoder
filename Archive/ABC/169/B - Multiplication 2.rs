use num_bigint::BigUint;
use num_traits::FromPrimitive;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [BigUint; n],
    }
    if v.iter().any(|x| *x == BigUint::from_i32(0).unwrap()) {
        println!("0");
    } else {
        let mut num = BigUint::from_i32(1).unwrap();
        for i in v.iter() {
            num *= i;
            if BigUint::from_u128(10_u128.pow(18)).unwrap() < num {
                println!("-1");
                return;
            }
        }
        println!("{}", num);
    }
}
