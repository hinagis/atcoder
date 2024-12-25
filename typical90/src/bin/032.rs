use proconio::{input, marker::Usize1};
const M: i32 = 1 << 30;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m]
    }
    let mut d = vec![vec![true; n]; n];
    for &(x, y) in &xy {
        d[x][y] = false;
        d[y][x] = false;
    }

    let r = dfs(&a, &d, n, 0, 0, n);
    println!("{}", if r >= M {-1} else {r});
}

fn dfs(a: &Vec<Vec<i32>>, d: &Vec<Vec<bool>>, n: usize, b: usize, j: usize, p: usize) -> i32 {
    if j < n {
        let mut r = M;
        for i in 0..n {
            if j == 0 || d[p][i] && ((b & (1 << i)) == 0) {
                r = r.min(a[i][j] + dfs(a, d, n, b | (1 << i), j + 1, i))
            }
        }
        r
    } else {
        0
    }
}