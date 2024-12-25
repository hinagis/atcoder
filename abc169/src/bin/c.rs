use proconio::input;
use num::*;

fn main() {
    input! {
        a: u64,
        b: f32,
    }
    let b = (b * 1000_f32) as u64;
    let a = bigint::BigUint::from_u64(a).unwrap();
    let b = bigint::BigUint::from_u64(b).unwrap();
    let r = a * b / bigint::BigUint::from_u64(1000).unwrap();

    println!("{}", r);
}
