use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    }
    let mut t = vec![vec![]; n];
    for &(a, b) in &ab {
        t[a].push(b);
        t[b].push(a);
    }
    let mut c = vec![true; n];
    let mut f = vec![true; n];
    f[0] = false;

    let mut g = false;

    let mut q = VecDeque::new();
    q.push_back(0);
    while ! q.is_empty() {
        let mut nq = VecDeque::new();
        for &i in &q {
            for &j in &t[i] {
                if f[j] {
                    f[j] = false;
                    c[j] = g;
                    nq.push_back(j);
                }
            }
        }
        g = ! g;
        q = nq;
    }

    let f = c.iter().filter(|&&f| f).count() * 2 >= n;
    let mut a = VecDeque::new();
    let mut j = 0;
    for _ in 0..n / 2 {
        while c[j] != f {
            j += 1;
        }
        a.push_back(j);
        j += 1;
    }
    println!("{}", a.iter().map(|i| (i + 1).to_string()).collect::<Vec<_>>().join(" "));
}
