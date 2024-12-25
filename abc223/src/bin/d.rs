use std::collections::BinaryHeap;
use std::cmp::Reverse;
use proconio::{input, marker::Usize1};
 
fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut t = vec![vec![]; n];
    let mut c = vec![0; n];
    for (a, b) in ab {
        t[a].push(b);
        c[b] += 1;
    }

    let mut p = BinaryHeap::new();
    for i in 0..n {
        if c[i] == 0 { p.push(Reverse(i)); }
    }
 
    let mut s = vec![];
    while let Some(Reverse(x)) = p.pop() {
        s.push(x);
        for &y in &t[x] {
            c[y] -= 1;
            if c[y] <= 0 {
                p.push(Reverse(y));
            }
        }
    }
    if s.len() < n {
        println!("-1");
    } else {
        println!("{}", s.iter().map(|i| (i + 1).to_string()).collect::<Vec<_>>().join(" "));
    }
}
