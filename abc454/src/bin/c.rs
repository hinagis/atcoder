use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut t = vec![vec![]; n];
    for _ in 0..m {
        I! {
            a: U,
            b: U
        }
        t[a].push(b);
    }
    let mut f = vec![false; n ];
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    f[0] = true;
    while let Some(i) = q.pop_front() {
        for &j in t[i].iter() {
            if !f[j] {
                f[j] = true;
                q.push_back(j);
            }
        }
    }
    println!("{}", f.iter().filter(|f| **f).count());
}
