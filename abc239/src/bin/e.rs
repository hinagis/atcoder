use proconio::{input as I, fastout, marker::Usize1 as U1};

#[fastout]
fn main() {
    I! {
        n: usize,
        q: usize,
        x: [u32; n],
        ab: [(U1, U1); n - 1],
        vk: [(U1, U1); q]
    }

    let mut t = vec![vec![]; n];
    for &(a, b) in &ab {
        t[a].push(b);
        t[b].push(a);
    }

    let mut y = vec![vec![]; n];
    dfs(&t, &x, &mut y, 0, 0);

    for &(v, k) in &vk {
        println!("{}", y[v][k]);
    }
}

fn dfs(t: &Vec<Vec<usize>>, x: &Vec<u32>, y: &mut Vec<Vec<u32>>, i: usize, p: usize) -> Vec<u32> {
    let mut c = vec![x[i]];
    for &j in &t[i] {
        if j != p {
            c.append(&mut dfs(t, x, y, j, i));
        }
    }
    c.sort_by(|a, b| b.cmp(&a));
    c.resize(20, 0);
    y[i] = c.clone();
    c
}
