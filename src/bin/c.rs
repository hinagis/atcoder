use proconio::{input as I};
fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let b = (0..n).fold(0, |s, i| s | (1 << i));
    let mut f = vec![0; m];
    for i in 0..m {
        I! {a: [u32]}
        for a in &a {
            f[i] |= 1 << (a - 1);
        }
    }
    fn dfs(b: u32, f: &Vec<u32>, p: u32, m: usize, i: usize) -> u32 {
        let mut c = 0;
        for j in i..m {
            if (p | f[j]) == b {
                c += 1;
            }
            c += dfs(b, f, p | f[j], m, j + 1);
        }
        c
    }
    println!("{}", dfs(b, &f, 0, m, 0));
}
