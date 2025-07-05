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
        let p = a.iter().filter(|&&e| e == a[0]).count();
        if p == n {
            println!("Yes");
            continue;
        }
        let m = a.iter().filter(|&&e| e == -a[0]).count();
        if p + m == n && p.min(m) == n / 2 {
            println!("Yes");
            continue;
        }

        let mut r = true;
        a.sort_by(|a, b| a.abs().cmp(&b.abs()));
        for i in 0..n - 2 {
            if a[i + 1] * a[i + 1] != a[i] * a[i + 2] {
                r = false;
                break;
            }
        }
        println!("{}", if r {"Yes"} else {"No"});
    }
}
