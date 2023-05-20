use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let mut p = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 's' {
                p.push((i, j));
            }
        }
    }
    for &(i, j) in p.iter() {
        if h - i >= 5 {
            if s[i + 1][j] == 'n'
            && s[i + 2][j] == 'u'
            && s[i + 3][j] == 'k'
            && s[i + 4][j] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i + 2, j + 1);
                println!("{} {}", i + 3, j + 1);
                println!("{} {}", i + 4, j + 1);
                println!("{} {}", i + 5, j + 1);
                return;
            }
            if w - j >= 5 {
                if s[i + 1][j + 1] == 'n'
                && s[i + 2][j + 2] == 'u'
                && s[i + 3][j + 3] == 'k'
                && s[i + 4][j + 4] == 'e' {
                    println!("{} {}", i + 1, j + 1);
                    println!("{} {}", i + 2, j + 2);
                    println!("{} {}", i + 3, j + 3);
                    println!("{} {}", i + 4, j + 4);
                    println!("{} {}", i + 5, j + 5);
                    return;
                }
            }
            if j >= 4 {
                if s[i + 1][j - 1] == 'n'
                && s[i + 2][j - 2] == 'u'
                && s[i + 3][j - 3] == 'k'
                && s[i + 4][j - 4] == 'e' {
                    println!("{} {}", i + 1, j + 1);
                    println!("{} {}", i + 2, j);
                    println!("{} {}", i + 3, j - 1);
                    println!("{} {}", i + 4, j - 2);
                    println!("{} {}", i + 5, j - 3);
                    return;
                }
            }
        }
        if i >= 4 {
            if s[i - 1][j] == 'n'
            && s[i - 2][j] == 'u'
            && s[i - 3][j] == 'k'
            && s[i - 4][j] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i, j + 1);
                println!("{} {}", i - 1, j + 1);
                println!("{} {}", i - 2, j + 1);
                println!("{} {}", i - 3, j + 1);
                return;
            }
            if w - j >= 5 {
                if s[i - 1][j + 1] == 'n'
                && s[i - 2][j + 2] == 'u'
                && s[i - 3][j + 3] == 'k'
                && s[i - 4][j + 4] == 'e' {
                    println!("{} {}", i + 1, j + 1);
                    println!("{} {}", i, j + 2);
                    println!("{} {}", i - 1, j + 3);
                    println!("{} {}", i - 2, j + 4);
                    println!("{} {}", i - 3, j + 5);
                    return;
                }
            }
            if j >= 4 {
                if s[i - 1][j - 1] == 'n'
                && s[i - 2][j - 2] == 'u'
                && s[i - 3][j - 3] == 'k'
                && s[i - 4][j - 4] == 'e' {
                    println!("{} {}", i + 1, j + 1);
                    println!("{} {}", i, j);
                    println!("{} {}", i - 1, j - 1);
                    println!("{} {}", i - 2, j - 2);
                    println!("{} {}", i - 3, j - 3);
                    return;
                }
            }
        }
        if w - j >= 5 {
            if s[i][j + 1] == 'n'
            && s[i][j + 2] == 'u'
            && s[i][j + 3] == 'k'
            && s[i][j + 4] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i + 1, j + 2);
                println!("{} {}", i + 1, j + 3);
                println!("{} {}", i + 1, j + 4);
                println!("{} {}", i + 1, j + 5);
                return;
            }
        }
        if j >= 4 {
            if s[i][j - 1] == 'n'
            && s[i][j - 2] == 'u'
            && s[i][j - 3] == 'k'
            && s[i][j - 4] == 'e' {
                println!("{} {}", i + 1, j + 1);
                println!("{} {}", i + 1, j);
                println!("{} {}", i + 1, j - 1);
                println!("{} {}", i + 1, j - 2);
                println!("{} {}", i + 1, j - 3);
                return;
            }
        }
    }
}
