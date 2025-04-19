use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let mut r = vec![std::collections::HashSet::new(); m];
    let mut p = vec![vec![]; n];
    for i in 0..m {
        I! {k: usize}
        for _ in 0..k {
            I! {a: U}
            r[i].insert(a);
            p[a].push(i);
        }
    }
    let mut c = 0;
    for _ in 0..n {
        I! {b: U}
        for &i in p[b].iter() {
            r[i].remove(&b);
            if r[i].is_empty() {
                c += 1;
            }
        }
        println!("{}", c);
    }
}
