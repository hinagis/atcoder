use proconio::input;

const M: usize = 998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut rng = vec![];
    for _ in 0..k {
        input!{ l: usize, r: usize };
        for j in l..=r {
            rng.push(j);
        }
    }
    rng.sort();

    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    for i in 1..n {
        for j in rng.iter() {
            if i + j > n {
                break;
            } else {
                dp[i + j] = (dp[i + j] + dp[i]) % M;
            }
        }
    }
    println!("{}", dp[n]);
}
