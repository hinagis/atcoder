use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            l: u32,
        }
        if n < 3 {
            I! {r: u32}
            println!("{}", if l * 2 < r {-1} else {2});
            continue;
        }
        I! {
            mut s: [u32; n - 2],
            r: u32
        }
        if l * 2 >= r {
            println!("2");
            continue;
        }
        s.sort();
        if l * 2 < s[0] || s[n - 3] * 2 < r {
            println!("-1");
            continue;
        }
        let mut p = l;
        let mut c = 2;
        let mut f = false;
        for i in 1..n - 2 {
            if p * 2 < s[i] {
                p = s[i - 1];
                c += 1;
                if p * 2 >= r {
                    println!("{}", c);
                    f = true;
                    break;
                }
                if p * 2 < s[i] {
                    println!("-1");
                    f = true;
                    break;
                }
            }
        }
        if !f {
            println!("{}", c + 1);
        }
    }
}
