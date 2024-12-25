use proconio::{input as I};

fn main() {
    I! {
        _: usize,
        mut s: String,
        q: usize,
        txc: [(u8, usize, char); q]
    }

    let mut p = 0;
    let mut l = 0;
    for i in 0..q {
        if let 2 | 3 = txc[i].0 {
            l = txc[i].0;
            p = i + 1;
        }
    }

    let mut s = match l {
        2 => s.to_lowercase(),
        3 => s.to_uppercase(),
        _ => s
    }.chars().collect::<Vec<_>>();

    for (i, &(t, x, c)) in txc.iter().enumerate() {
        if t == 1 {
            let x = x - 1;
            if i >= p {
                s[x] = c;
            } else if l == 2 {
                s[x] = c.to_ascii_lowercase();
            } else if l == 3 {
                s[x] = c.to_ascii_uppercase();
            }
        }
    }
    println!("{}", s.iter().collect::<String>());
}
