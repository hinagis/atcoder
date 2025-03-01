use nalgebra::QR;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        x: usize,
    }
    let mut t = [vec![vec![]; n], vec![vec![]; n]];
    for _ in 0..m {
        I! {
            u: U,
            v: U,
        }
        t[0][u].push(v);
        t[1][v].push(u);
    }
    let mut f = vec![true; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((i, c, d)) = q.pop_front() {
        f[i] = false;
        for &j in t[d][i].iter() {
            if j == n - 1 {
                println!("{}", c + 1);
                return;
            }
            if f[j] {
                q.push_back((j, c + 1, d));
            }
        }
        for &j in t[d ^ 1][i].iter() {
            if j == n - 1 {
                println!("{}", c + 1);
                return;
            }
            if f[j] {
                q.push_back((j, c + 1, d));
            }
        }
    }
    println!("{}", n);
}
