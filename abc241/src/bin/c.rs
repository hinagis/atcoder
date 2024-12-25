use proconio::{input as I, marker::Chars};

fn main() {
    I! {
        n: usize,
        s: [Chars; n]
    }

    let mut c = vec![vec![[(0, 0); 4]; n]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                for k in 0..4 {
                    c[i][j][k].0 += 1;
                }
                for k in 1..6 {
                    if i + k < n {
                        c[i + k][j][0].0 += 1;
                        if j + k < n {
                            c[i + k][j + k][1].0 += 1;
                        }
                        if j >= k {
                            c[i + k][j - k][2].0 += 1;
                        }
                    }
                    if j + k < n {
                        c[i][j + k][3].0 += 1;
                    }
                }
            } else {
                for k in 0..4 {
                    c[i][j][k].1 += 1;
                }
                for k in 1..6 {
                    if i + k < n {
                        c[i + k][j][0].1 += 1;
                        if j + k < n {
                            c[i + k][j + k][1].1 += 1;
                        }
                        if j >= k {
                            c[i + k][j - k][2].1 += 1;
                        }
                    }
                    if j + k < n {
                        c[i][j + k][3].1 += 1;
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if c[i][j][0].0 >= 4
            || (c[i][j][1].0 >= 4 && c[i][j][1].0 + c[i][j][1].1 == 6)
            || (c[i][j][2].0 >= 4 && c[i][j][2].0 + c[i][j][2].1 == 6)
            || c[i][j][3].0 >= 4
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
