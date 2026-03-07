use proconio::{input as I, fastout as F, marker::Usize1 as U};
use std::collections::HashMap as H;

#[F]
fn main() {
    I! {
        n: usize,
        a: [u64; n]
    }

    let mut t = vec![vec![]; n];
    for _ in 1..n {
        I! {
            u: U,
            v: U,
        }
        t[u].push(v);
        t[v].push(u);
    }
    let mut f = vec![(false, false); n];
    let mut h = H::new();
    dfs(&a, &t, &mut f, &mut h, 0, false);
    for i in 0..n {
        println!("{}", if f[i].1 {"Yes"} else {"No"});
    }
}

fn dfs(a: &Vec<u64>, t: &Vec<Vec<usize>>, f: &mut Vec<(bool, bool)>, h: &mut H<u64, u32>, i: usize, p: bool) {
    if f[i].0 {return}
    f[i].0 = true;
    let n = {
        let t = h.entry(a[i]).or_insert(0);
        *t += 1;
        p || *t > 1
    };
    f[i].1 = n;
    for &j in &t[i] {
        dfs(a, t, f, h, j, n);
    }
    *h.get_mut(&a[i]).unwrap() -= 1;
    f[i].0 = false;
}
