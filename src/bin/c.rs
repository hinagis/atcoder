fn main() {
    proconio::input! {n: u32}
    let s = 3usize.pow(n);
    let mut g = vec![vec!['#'; s]; s];
    for i in 0..n {
        let b = 3usize.pow(i);
        for r in 0..3 {
            for c in 0..3 {
                if r == 0 && c == 0 {
                    continue;
                } else if r == 1 && c == 1 {
                    for u in 0..b {
                        for v in 0..b {
                            g[r * b + u][c * b + v] = '.';
                        }
                    }
                } else {
                    for u in 0..b {
                        for v in 0..b {
                            g[r * b + u][c * b + v] = g[u][v];
                        }
                    }
                }
            }
        }
    }
    println!("{}", g.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
}
