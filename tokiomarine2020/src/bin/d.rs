use proconio::input;
use std::cmp::{max, min};

const MAX_L: usize = 100_000;
const BASE_SIZE: usize = 1 << 10;
const ALL_L: std::ops::RangeInclusive<usize> = 0..=MAX_L;

fn main() {
    input! {
        n: usize,
        vw: [(u32, u32); n],
        q: usize,
        ul: [(usize, usize); q],
    }
    let dp = make_dp(&vw);
    for (u, l) in ul {
        println!("{}", calc(&dp, &vw, u - 1, l));
    }
}

fn make_dp(vw: &Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    let mut dp = Vec::<Vec<u32>>::new();

    dp.push({
        let w = vw[0].1 as usize;
        ALL_L
        .map(|i| if i < w { 0 } else { vw[0].0 })
        .collect()
    });

    let n = min(vw.len(), BASE_SIZE - 1);
    for c in 1..n {
        let p = (c - 1) / 2;
        dp.push({
            let w = vw[c].1 as usize;
            let v = vw[c].0;
            ALL_L
            .map(|i| {
                max(dp[p][i], if i < w { 0 } else { dp[p][i - w] + v })
            })
            .collect()
        });
    }

    dp
}

fn calc(
    dp: &Vec<Vec<u32>>,
    vw: &Vec<(u32, u32)>,
    u: usize, 
    l: usize,
) -> u32 {
    if u < dp.len() {
        dp[u][l]
    } else {
        let p = (u - 1) / 2;
        let w = vw[u].1 as usize;
        let sum1 = calc(dp, vw, p, l);
        let sum2 = if l < w { 0 } else { calc(dp, vw, p, l - w) + vw[u].0 };
        max(sum1, sum2)
    }
}
