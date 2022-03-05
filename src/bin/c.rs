const M: u64 = 998244353;

fn main() {
    proconio::input! {n: usize}
    let mut v = vec![[1; 9]; n];
    for i in 1..n {
        v[i][0] = v[i - 1][1];
        for j in 1..8 {
            v[i][j] = v[i - 1][j - 1] + v[i - 1][j + 1];
            v[i][j] %= M;
        }
        v[i][8] = v[i - 1][7];
        for j in 0..9 {
            v[i][j] += v[i - 1][j];
            v[i][j] %= M;
        }
    }

    let mut r = 0;
    for j in 0..9 {
        r += v[n - 1][j];
        r %= M;
    }
    println!("{}", r);
}
