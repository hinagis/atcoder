use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
    }
    let mut t = vec![u64::MAX; n];
    for _ in 0..m {
        I! {
            k: usize,
            c: u64,
            a: [U; k]
        }
        for i in 0..k {
            t[a[i]] = c.min(t[a[i]]);
        }
    }

    let mut r = 0;
    for &c in &t {
        if c == u64::MAX {
            println!("-1");
            return;
        }
        r += c;
    }

    println!("{}", r - t.iter().min().unwrap());
}
