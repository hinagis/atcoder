use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C,
    }

    let mut f = 0;
    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        I! {w: u64}
        a.push((w, s[i]));
        if s[i] == '1' {
            f += 1;
        }
    }
    a.sort();

    let mut m = f;
    let mut i = 0;
    while i < n {
        let w = a[i].0;
        while i < n && a[i].0 == w {
            if a[i].1 == '0' {
                f += 1;
            } else {
                f -= 1;
            }
            i += 1;
        }
        m = m.max(f);
    }

    println!("{}", m);
}
