use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        p: [U; n],
        q: [U; n]
    }
    let mut d = std::collections::HashMap::new();
    for i in 0..n {
        d.insert(q[i], i);
    }
    println!("{}", (0..n).map(|i| q[p[*d.get(&i).unwrap()]] + 1).join(" "));
}
