use proconio::{input, fastout, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, u64); n - 1]
    }
    let mut t = vec![vec![]; n];
    for i in 0..n - 1 {
        let (u, v, w) = uvw[i];
        t[u].push((v, w));
        t[v].push((u, w));
    }
    let mut r = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut m = 0;
            let mut f = vec![true; n];
            f[i] = false;
            let mut q = std::collections::VecDeque::new();
            q.push_back((i, 0));
            while let Some(e) = q.pop_front() {
                for &(v, w) in &t[e.0] {
                    if v == j {
                        m = w;
                        q.clear();
                        break;
                    }
                    if f[v] {
                        q.push_back((v, e.1.max(w)));
                    }
                }
            }
            r += m;
        }
    }
    println!("{}", r);
}
