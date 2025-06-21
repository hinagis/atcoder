use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        x: [i64; n]
    }
    let mut e = vec![std::collections::BTreeSet::new(); n];
    for _ in 0..n - 1{
        I! {
            u: U,
            v: U,
            w: i64
        }
        e[u].insert((v, w));
        e[v].insert((u, w));
    }
    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if e[i].len() == 1 {
            q.push_back(i);
        }
    }
    println!("{}", n);
}
