use proconio::{input, marker::Usize1};
use std::collections::{HashSet, VecDeque};

const S: usize = 8;

fn main() {
    input! {m: usize}
    let mut g = vec![vec![]; 9];
    for _ in 0..m {
        input! {u: Usize1, v: Usize1}
        g[u].push(v);
        g[v].push(u);
    }

    let mut b = [S; 9];
    for i in 0..8 {
        input! {p: Usize1}
        b[p] = i;
    }

    let mut f = HashSet::new();
    let mut q = VecDeque::new();

    f.insert(b);
    q.push_back((b, b.iter().position(|&p| p == S).unwrap(), 0));

    while let Some((b, u, c)) = q.pop_front() {
        if b == [0, 1, 2, 3, 4, 5, 6, 7, 8] {
            println!("{}", c);
            return;
        }

        for &v in &g[u] {
            let mut b = b;
            b.swap(u, v);
            if ! f.contains(&b) {
                f.insert(b);
                q.push_back((b, v, c + 1));
            }
        }
    }

    println!("-1");
}
