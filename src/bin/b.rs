use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        r: usize,
        c: usize,
        b: [C; r]
    }
    let mut a = vec![vec!['.'; c]; r];
    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '#' {
                a[i][j] = '#';
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            let t = b[i][j];
            if t >= '1' && t <= '9' {
                let t = (t as u8 - b'0') as usize;
                for u in 0..=t {
                    if i + u < r {
                        for v in 0..=t - u {
                            if j + v < c {
                                a[i + u][j + v] = '.';
                            }
                            if j >= v {
                                a[i + u][j - v] = '.';
                            }
                        }
                    }
                    if i >= u {
                        for v in 0..=t - u {
                            if j + v < c {
                                a[i - u][j + v] = '.';
                            }
                            if j >= v {
                                a[i - u][j - v] = '.';
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", a.iter().map(|a| a.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
}
