use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    }
    let mut v = vec![vec![0; w]; h];
    for i in (0..h - 1).rev() {
        v[i][w - 1] = v[i + 1][w - 1] + if (i + w - 1) % 2 == 0 {1} else {-1} * if a[i + 1][w - 1] == '+' {1} else {-1}
    }
    for j in (0..w - 1).rev() {
        v[h - 1][j] = v[h - 1][j + 1] + if (h - 1 + j) % 2 == 0 {1} else {-1} * if a[h - 1][j + 1] == '+' {1} else {-1}
    }
    for i in (0..h - 1).rev() {
        for j in (0..w - 1).rev() {
            let p = if (i + j) % 2 == 0 {1} else {-1};
            let vi = v[i + 1][j] + p * if a[i + 1][j] == '+' {1} else {-1};
            let vj = v[i][j + 1] + p * if a[i][j + 1] == '+' {1} else {-1};
            v[i][j] = if (i + j) % 2 == 0 {vi.max(vj)} else {vi.min(vj)}
        }
    }
    println!("{}", if v[0][0] == 0 {"Draw"} else if v[0][0] > 0 {"Takahashi"} else {"Aoki"});
}
