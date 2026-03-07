use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut t = vec![vec![]; n];
    let mut f = vec![false; n];
    for _ in 0..m {
        I! {
            x: U,
            y: U
        }
        t[y].push(x);
    }
    I! {q: usize}
    for _ in 0..q {
        I! {
            k: u8,
            v: U
        }
        if k == 1 {
            if f[v] {continue}
            f[v] = true;
            let mut b = std::collections::VecDeque::new();
            b.push_back(v);
            while let Some(v) = b.pop_front() {
                for &u in &t[v] {
                    if !f[u] {
                        f[u] = true;
                        b.push_back(u);
                    }
                }
            }
        } else {
            println!("{}", if f[v] {"Yes"} else {"No"});
        }
    }
}
