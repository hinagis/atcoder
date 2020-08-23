use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n * 3],
    }
    let mut dp = vec![vec![vec![0; n]; n]; n + 1];
    let mut q = HashSet::new();
    q.insert((a[0], a[1]));
    for i in 1..n {
        let mut nq = HashSet::new();
        for (a1, a2) in q {
            let a3 = a[i * 3 - 1];
            let a4 = a[i * 3];
            let a5 = a[i * 3 + 1];
            let a6 = a[i * 3 + 2];
            let p = if a6 == a1 {
                if        a6 == a2 { Some((a1, a2, a3, a4, a5))
                } else if a6 == a3 { Some((a1, a3, a2, a4, a5))
                } else if a6 == a4 { Some((a1, a4, a3, a2, a5))
                } else if a6 == a5 { Some((a1, a5, a3, a4, a2))
                } else { None
                }
            } else if a6 == a2 {
                if        a6 == a3 { Some((a2, a3, a1, a4, a5))
                } else if a6 == a4 { Some((a2, a4, a3, a1, a5))
                } else if a6 == a5 { Some((a2, a5, a3, a4, a5))
                } else { None
                }
            } else if a6 == a3 {
                if        a6 == a4 { Some((a3, a4, a1, a2, a5))
                } else if a6 == a5 { Some((a3, a5, a1, a4, a2))
                } else { None
                }
            } else if a6 == a4 && a6 == a5 {
                Some((a4, a5, a3, a1, a2))
            } else {
                None
            };

            if let Some(p) = p {
                dp[i + 1][p.0][p.1] = dp[i + 1][p.0][p.1].max(dp[i][a1][a2] + if p.2 == p.3 && p.2 == p.4 { 2 } else { 1 });
                nq.insert((p.0, p.1));
            } else {
                let p = if a1 == a2 && a1 == a3 { Some((a4, a5, a1, a2, a3))
                } else if  a1 == a2 && a1 == a4 { Some((a3, a5, a1, a2, a4))
                } else if  a1 == a2 && a1 == a5 { Some((a3, a4, a1, a2, a5))
                } else if  a1 == a3 && a1 == a4 { Some((a2, a5, a1, a3, a4))
                } else if  a1 == a3 && a1 == a5 { Some((a2, a4, a1, a3, a5))
                } else if  a1 == a4 && a1 == a5 { Some((a2, a3, a1, a4, a5))
                } else if  a2 == a3 && a2 == a4 { Some((a1, a5, a2, a3, a4))
                } else if  a2 == a3 && a2 == a5 { Some((a1, a4, a2, a3, a5))
                } else if  a2 == a4 && a2 == a5 { Some((a1, a3, a2, a4, a5))
                } else if  a3 == a4 && a3 == a5 { Some((a1, a2, a3, a4, a5))
                } else {   None
                };
                if let Some(p) = p {
                    dp[i + 1][p.0][p.1] = dp[i + 1][p.0][p.1].max(dp[i][a1][a2] + 1);
                    nq.insert((p.0, p.1));
                } else {
                    let (x, y) = (a1, a2); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a1, a3); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a1, a4); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a1, a5); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a2, a3); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a2, a4); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a2, a5); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a3, a4); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a3, a5); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                    let (x, y) = (a4, a5); dp[i + 1][x][y] = dp[i + 1][x][y].max(dp[i][a1][a2]); nq.insert((x, y));
                }
            }
        }
        q = nq;
    }

    let mut r = 0;
    for qi in q {
        r = r.max(dp[n][qi.0][qi.1]);
    }

    dbg!(&dp);
    println!("{}", r);
}
