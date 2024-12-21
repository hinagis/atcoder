use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        k: usize,
        s: [C; h]
    }

    let mut m = std::usize::MAX;

    for i in 0..h {
        let mut t = 0;
        let mut r = k;
        for j in 0..w {
            if s[i][j] == 'x' {
                r = k;
                t = j + 1;
            } else {
                if s[i][j] == 'o' {
                    r -= 1;
                }
                if j - t >= k - 1 {
                    m = m.min(r);
                    if s[i][t] == 'o' {
                        r += 1;
                    }
                    t += 1;
                }
            }
        }
    }

    for j in 0..w {
        let mut t = 0;
        let mut r = k;
        for i in 0..h {
            if s[i][j] == 'x' {
                r = k;
                t = i + 1;
            } else {
                if s[i][j] == 'o' {
                    r -= 1;
                }
                if i - t >= k - 1 {
                    m = m.min(r);
                    if s[t][j] == 'o' {
                        r += 1;
                    }
                    t += 1;
                }
            }
        }
    }

    println!("{}", if m == std::usize::MAX {-1} else {m as isize});
}
