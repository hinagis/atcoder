use itertools::Itertools;
use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        r: C,
        c: C
    }

    let b = vec![vec!['.'; n]; n];
    let f = vec![vec![0x7; n + 1]; n + 1];
    if !calc(&r, &c, &b, &f, n, 0, 0) {
        println!("No");
    }
}

fn calc(r: &Vec<char>, c: &Vec<char>,b: &Vec<Vec<char>>, f: &Vec<Vec<u8>>, n: usize, i: usize, mut j: usize) -> bool {
    if i >= n {
        for i in 0..n {
            for j in 0..n {
                if b[i][j] == r[i] {break}
                else if b[i][j] == '.' {continue}
                else {return false}
            }
        }
        for j in 0..n {
            for i in 0..n {
                if b[i][j] == c[j] {break}
                else if b[i][j] == '.' {continue}
                else {return false}
            }
        }
        println!("Yes\n{}", b.iter().map(|s| s.iter().collect::<String>()).join("\n"));
        return true;
    } else if f[i][n] == 0 {
        if calc(r, c, &b, &f, n, i + 1, 0) {return true}
    } else {
        while j < n {
            for k in 0..3 {
                let g = 1 << k;
                if (f[i][n] & g == g) && (f[n][j] & g == g) && (f[i][j] & g == g) {
                    let mut b = b.clone();
                    let mut f = f.clone();
                    b[i][j] = (b'A' + k as u8) as char;
                    for i in 0..=n {
                        f[i][j] &= !g;
                    }
                    for j in 0..=n {
                        f[i][j] &= !g;
                    }
                    if calc(r, c, &b, &f, n, i, j + 1) {return true}
                }
            }
            j += 1;
        }
    }
    false
}
