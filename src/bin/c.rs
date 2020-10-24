use std::collections::{HashMap, HashSet};
use rand::{thread_rng, Rng};

const N: usize = 13;

fn main() {
    let az = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let mut h = HashMap::new();
    for &c in &az {
        h.insert(c, "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<_>>());
    }

    let solve = |dp: &mut Vec<Vec<char>>| -> bool {
        let mut rng = thread_rng();
        let mut h = h.clone();
        for j in 1..N {
            let cl = dp[0][j - 1];
            if h[&cl].len() == 0 {
                return false;
            }
            let k = rng.gen_range(0, h[&cl].len());
            let nc = h[&cl].iter().nth(k).unwrap().clone();
            dp[0][j] = nc;
            h.get_mut(&cl).unwrap().remove(&nc);
        }
        for i in 1..N {
            {   let cu = dp[i - 1][0];
                if h[&cu].len() == 0 {
                    return false;
                }
                let k = rng.gen_range(0, h[&cu].len());
                let nc = h[&cu].iter().nth(k).unwrap().clone();
                dp[i][0] = nc;
                h.get_mut(&cu).unwrap().remove(&nc);
            }
            for j in 1..N {
                let cu = dp[i - 1][j];
                let cl = dp[i][j - 1];
                dp[i][j] = if cu == cl {
                    if h[&cu].len() == 0 {
                        return false;
                    }
                    let k = rng.gen_range(0, h[&cu].len());
                    let nc = h[&cu].iter().nth(k).unwrap().clone();
                    h.get_mut(&cu).unwrap().remove(&nc);
                    nc
                } else {
                    let ps = &h[&cu] & &h[&cl];
                    if ps.len() == 0 {
                        return false;
                    }
                    let k = rng.gen_range(0, ps.len());
                    let nc = ps.iter().nth(k).unwrap().clone();
                    h.get_mut(&cu).unwrap().remove(&nc);
                    h.get_mut(&cl).unwrap().remove(&nc);
                    nc
                }
            }
        }
        true
    };

    let mut dp = vec![vec!['a'; N]; N];
    while !solve(&mut dp) {
    }

    println!("{}", N);
    for i in 0..N {
        println!("{}", dp[i].iter().collect::<String>());
    }
}
