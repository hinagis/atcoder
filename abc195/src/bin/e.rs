use proconio::{input, marker::{Bytes, Chars}};

fn main() {
    input! {
        n: usize,
        s: Bytes,
        x: Chars,
    }
    let s: Vec<u8> = s.iter().map(|c| c - b'0').collect();

    let mut dp = 1;
    for i in (0..n).rev() {
        let mut r = 0;
        for j in 0..7 {
            let m = ((j * 10 % 7), (j * 10 + s[i]) % 7);
            let f = ((dp >> m.0) & 1 == 1, (dp >> m.1) & 1 == 1);
            let f = if x[i] == 'T' {f.0 || f.1} else {f.0 && f.1};
            if f {
                r |= 1 << j;
            }
        }
        dp = r;
    }

    println!("{}", if dp & 1 == 1 {"Takahashi"} else {"Aoki"});
}
