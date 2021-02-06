use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut c = 0;
    let mut p = '.';
    for j in 1..w {
        let n = s[1][j];
        if n != p {
            c += 1;
            p = n;
        }
    }
    dbg!(&c);
    for i in 1..(h - 2) {
        let mut p = ('.', '.');
        for j in 1..w {
            let n = (s[i][j], s[i + 1][j]);
            if n != p {
                if n.0 != n.1 || p.0 != p.1 {
                    c += 1;
                }
                p = n;
            }
        }
        dbg!(&c);
    }
    let mut p = '.';
    for j in 1..w {
        let n = s[h - 2][j];
        if n != p {
            c += 1;
            p = n;
        }
    }
    dbg!(&c);
    println!("{}", c);
}
