use proconio::{input as I, fastout, marker::Usize1 as U1};

#[fastout]
fn main() {
    I! {
        n: usize,
        uv: [(U1, U1); n - 1]
    }

    let mut t = vec![(vec![], (0, 0)); n];
    for &(u, v) in &uv {
        t[u].0.push(v);
        t[v].0.push(u);
    }

    dfs(&mut t, 0, 0, 1);

    for i in 0..n {
        println!("{} {}", (t[i].1).0, (t[i].1).1);
    }
}

fn dfs(t: &mut Vec<(Vec<usize>, (usize, usize))>, p: usize, i: usize, v: usize) {
    if p != i && t[i].0.len() == 1 {
        t[i].1 = (v, v);
    } else {
        t[i].1 = (usize::max_value(), 0);
        let mut l = v;
        for j in 0..t[i].0.len() {
            if t[i].0[j] != p {
                dfs(t, i, t[i].0[j], l);
                (t[i].1).0 = (t[i].1).0.min((t[t[i].0[j]].1).0);
                (t[i].1).1 = (t[i].1).1.max((t[t[i].0[j]].1).1);
                l = (t[i].1).1 + 1;
            }
        }
    }
}
