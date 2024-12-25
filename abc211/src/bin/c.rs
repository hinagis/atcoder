use proconio::{input, marker::Chars};
const M: u64 = 1000_000_007;

fn main() {
    input! {s: Chars}
    let t: Vec<char> = " chokudai".chars().collect();
    let mut v = vec![0; t.len()];
    v[0] = 1;
    for i in 0..s.len() {
        for j in 1..t.len() {
            if s[i] == t[j] {
                v[j] = (v[j] + v[j - 1]) % M;
                break;
            }
        }
    }
    println!("{}", v[t.len() - 1]);
//    let mut dp = vec![None; s.len()];
//    println!("{}", dfs(&s, &t, &mut dp, 0, 0));
}

/*
fn dfs(s: &Vec<char>, t: &Vec<char>, dp: &mut Vec<Option<u64>>, i: usize, mut j: usize) -> u64 {
    let mut r = 0;
    while j < s.len() {
        if s[j] == t[i] {
            r += if let Some(c) = dp[j] {
                c
            } else {
                let c = if i + 1 == t.len() {1} else {dfs(s, t, dp, i + 1, j + 1)};
                dp[j] = Some(c % M);
                c
            };
            r %= M;
        }
        j += 1;
    }
    r
}
*/
