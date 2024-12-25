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

    let calc = |s: usize| {
        let (mut m, mut c) = (s, 0);
    
        let mut f = vec![true; n];
        f[s] = false;
        let mut q = VecDeque::new();
        q.push_back(s);
        while ! q.is_empty() {
            m = q[0];
            c += 1;
    
            let mut nq = VecDeque::new();
            for &i in &q {
                for &j in &t[i] {
                    if f[j] {
                        f[j] = false;
                        nq.push_back(j);
                    }
                }
            }
            q = nq;
        }
    
        (m, c)
    };

    println!("{}", calc(calc(0).0).1);
}
