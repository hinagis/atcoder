use std::collections::{HashSet as H, VecDeque as Q};
fn main() {
    proconio::input! {
        n: usize,
        d: i64,
        p: [(i64, i64); n]
    }

    let mut h = H::new();
    for i in 1..n {
        h.insert(i);
    }

    let mut f = vec![false; n];
    f[0] = true;
    let mut q = Q::new();
    q.push_front(0);
    while let Some(i) = q.pop_front() {
        for j in h.clone() {
            if (p[i].0 - p[j].0).pow(2) + (p[i].1 - p[j].1).pow(2) <= d * d {
                h.remove(&j);
                q.push_front(j);
                f[j] = true;
            }
        }
    } 
    for i in 0..n {
        println!("{}", if f[i] {"Yes"} else {"No"});
    }
}
