fn main() {
    proconio::input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n]
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..s {
            if dp[i][j] {
                if j + a <= s {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= s {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }
    if dp[n][s] {
        let mut r = String::new();
        let mut i = n;
        let mut j = s;
        while i > 0 {
            i -= 1;
            let (a, b) = ab[i];
            if j >= a && dp[i][j - a] {
                r.insert(0, 'H');
                j -= a;
            } else {
                r.insert(0, 'T');
                j -= b;
            }
        }
        println!("Yes\n{}", r);
    } else {
        println!("No");
    }
}
