const M: u32 = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut v = vec![vec![0; k + 1]; n + 1];
    for j in 1..=m {
        v[1][j] = 1;
    }
    for i in 2..=n {
        for j in 1..=m {
            for l in 0..k {
                if j + l <= k {
                    v[i][j + l] += v[i - 1][l];
                    v[i][j + l] %= M;
                }
            }
        }
    }
    println!("{}", v[n].iter().fold(0, |s, c| (s + c) % M));
}
