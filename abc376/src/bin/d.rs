use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        I! {
            a: U,
            b: U
        }
        g[a].push(b);
    }
    let mut r = std::u32::MAX;
    let mut f = vec![true; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0));
    while let Some((i, c)) = q.pop_front() {
        f[i] = false;
        for &j in g[i].iter() {
            if j == 0 {
                r = r.min(c + 1);
            }
            if f[j] {
                q.push_back((j, c + 1));
            }
        }
    }
    if r == std::u32::MAX {
        println!("-1");
    } else {
        println!("{}", r);
    }
}
