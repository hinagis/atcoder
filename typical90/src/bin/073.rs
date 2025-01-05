use proconio::{input as I, marker::Usize1 as U};

const M: u64 = 1_000_000_007;

fn main() {
    I! {
        n: usize,
        c: [char; n]
    }
    let mut f = vec![[0; 3]; n];
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        I! {
            a: U,
            b: U
        }
        g[a].push(b);
        g[b].push(a);
    }
    dfs(&c, &g, &mut f, 0, 0);
    dbg!(&f);
    println!("{}", f[0][2]);
}

fn dfs(c: &Vec<char>, g: &Vec<Vec<usize>>, f: &mut Vec<[u64; 3]>, p: usize, i: usize) {
    for &j in g[i].iter().filter(|&&j| j != p) {
        dfs(c, g, f, i, j);
    }
    let t = if c[i] == 'a' {0} else {1};
    let (mut u, mut v) = (1, 1);
    for &j in g[i].iter().filter(|&&j| j != p) {
        u *= (f[j][t] + f[j][2]) % M;
        u %= M;
        v *= (f[j][0] + f[j][1] + 2 * f[j][2]) % M;
        v %= M;
    }
    f[i][t] = u;
    f[i][2] = (v + M - u) % M;
}
