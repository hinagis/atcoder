use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n];
    for i in 0..m {
        I! {
            a: U,
            b: U,
            c: u64
        }
        g[a].push((b, c, i));
        g[b].push((a, c, i));
    }

    let mut d = vec![(u64::max_value(), 0); n];
    d[0] = (0, 0);

    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &(v, c, i) in &g[u] {
            if d[u].0 + c < d[v].0 {
                d[v] = (d[u].0 + c, i);
                q.push_back(v);
            }
        }
    }

    println!("{}", d[1..].iter().map(|&(_, i)| (i + 1).to_string()).collect::<Vec<_>>().join(" "));
}
