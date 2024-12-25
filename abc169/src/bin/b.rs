//use proconio::{input, fastout};
use proconio::input;
use num::*;

const L: u64 = 1000000000000000000;

//#[fastout]
fn main() {
    input! {
        n: usize,
        mut d: [u64; n],
    }
    let lim = bigint::BigUint::from_u64(L).unwrap();
    let mut r = bigint::BigUint::from_u64(1).unwrap();
    d.sort();
    if d[0] == 0 {
        println!("0");
        return;
    }
    for a in d {
        r *= a;
        if r > lim {
            println!("-1");
            return;
        }
    }
    println!("{}", r);
}
