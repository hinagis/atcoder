use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {
        I! {
            n: usize,
            mut a: [i64; n]
        }
        if n == 2 {
            println!("Yes");
            continue;
        }
        let p = a.iter().filter(|&&e| e > 0).count();
        let m = a.iter().filter(|&&e| e < 0).count();
        let mut r = true;
        if p == 0 || m == 0 {
            a.sort();
            for i in 0..n - 2 {
                if a[i + 1] * a[i + 1] != a[i] * a[i + 2] {
                    r = false;
                    break;
                }
            }
        } else if p.max(m) - p.min(m) > 1 {
            r = false;
        } else {
            a.sort_by(|a, b| a.abs().cmp(&b.abs()));
            for i in 0..n - 2 {
                if a[i] < 0 && a[i + 1] < 0 || a[i + 1] < 0 && a[i + 2] < 0 ||
                    a[i] > 0 && a[i + 1] > 0 || a[i + 1] > 0 && a[i + 2] > 0 ||
                    a[i + 1] * a[i + 1] != a[i] * a[i + 2] {
                    r = false;
                    break;
                }
            }
        }
        println!("{}", if r {"Yes"} else {"No"});
    }
}
