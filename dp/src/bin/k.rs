use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = vec![false; k + 1];

    for i in 1..=k {
        for j in 0..n {
            if i >= a[j] && dp[i - a[j]] == false {
                dp[i] = true;
            }
        }
    }
    println!("{}", if dp[k] {"First"} else { "Second" });
}
