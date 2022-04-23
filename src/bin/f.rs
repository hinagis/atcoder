use proconio::{input as I, marker::Usize1 as U1};

fn main() {
    I! {n: usize, m: usize}

    let mut e = vec![vec![]; n];
    let mut c = vec![0; n];
    for _ in 0..m {
        I! {u: U1, v: U1}
        e[v].push(u);
        c[u] += 1;
    }

    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if c[i] == 0 {
            q.push_back(i);
        }
    }

    let mut r = n;
    while let Some(f) = q.pop_front() {
        r -= 1;
        for &u in &e[f] {
            c[u] -= 1;
            if c[u] == 0 {
                q.push_back(u);
            }
        }
    }

    println!("{}", r);
}
