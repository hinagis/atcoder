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
        let (a, b) = if a < b {(a, b)} else {(b, a)};
        t[a].push(b);
    }

    fn dfs(t: &Vec<Vec<usize>>, x: &Vec<u32>, y: &mut Vec<Vec<u32>>, i: usize) {
        for j in 0..t[i].len() {
            dfs(t, x, y, t[i][j]);
        }
        let mut c = vec![x[i]];
        for j in 0..t[i].len() {
            for &y in &y[t[i][j]] {
                c.push(y);
            }
            c.sort_by(|a, b| b.cmp(&a));
            c.truncate(20);
        }
        y[i] = c;
    }

    let mut y = vec![vec![0; 20]; n];
    dfs(&t, &x, &mut y, 0);

    for &(v, k) in &vk {
        println!("{}", y[v][k]);
    }
}
