//use proconio::{input, fastout};
use proconio::input;
use itertools::*;
use std::collections::HashMap;

const M: u128 = 998244353;

//#[fastout]
fn main() {
    input! {
        n: usize,
        s: u128,
        mut a: [u128; n],
    }

    let mut dp = HashMap::new();
    let mut r = 0;
    for e in 0..n {
        if a[e] == s {
            dp.insert(vec!(e), 1);
        } else {
            dp.insert(vec!(e), 0);
        }
    }
    for k in 2..(n + 1) {
        let c = (0..n).combinations(k);
        for ci in c {
            for _e in dp.keys() {
//                if ci.is_subset(&e)
                {

                }
            }
            if let Some(v) = dp.get(&ci) {
                r += v;
            } else {
                let mut sum = 0;
                for e in &ci {
                    sum += a[*e];
                }
                if sum == s {
                    r += 1;
                }
            }
        }
    }
    println!("{}", r % M);
}
