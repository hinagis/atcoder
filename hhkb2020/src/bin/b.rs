use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut r = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            if s[i][j] == '.' {
                if s[i][j + 1] == '.' {
                    r += 1;
                }
                if s[i + 1][j] == '.' {
                    r += 1;
                }
            }
        }
        if s[i][w - 1] == '.' && s[i + 1][w - 1] == '.' {
            r += 1;
        }
    }
    let i = h - 1;
    for j in 0..(w - 1) {
        if s[i][j] == '.' && s[i][j + 1] == '.' {
            r += 1;
        }
    }

    println!("{}", r);
}
