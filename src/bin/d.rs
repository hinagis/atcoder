//use proconio::{input, fastout};
use proconio::input;
use std::collections::HashMap;

//#[fastout]
fn main() {
    input! {
        n: usize, k: u64,
        a: [usize; n],
    }

    let mut h = HashMap::new();
    let mut count = 0;
    let mut p = 1;
    while count < k {
        h.insert(p, count);
        count += 1;

        let next = a[p - 1];
        if let Some(s) = h.get(&next) {
            let loops = count - s;
            let k = (k - s) % loops + s;
            let (p, _) = h.iter().find(|(_, v)| **v == k).unwrap();
            println!("{}", p);
            return;
        }

        p = next;
    }
    println!("{}", p);
}
