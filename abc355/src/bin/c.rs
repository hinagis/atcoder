use proconio::{input as I, marker::Usize1 as U};
fn main() {
    I! {
        n: usize,
        a: [U]
    }
    let mut s = 1;
    let mut r = vec![n; n];
    let mut c = vec![n; n];
    let mut x = (n, n);
    for a in a {
        let i = a / n;
        let j = a % n;
        r[i] -= 1;
        c[j] -= 1;
        if i == j {
            x.0 -= 1;
        }
        if i == n - 1 - j {
            x.1 -= 1;
        }
        if r[i] == 0 || c[j] == 0 || x.0 == 0 || x.1 == 0 {
            println!("{}", s);
            return;
        }
        s += 1;
    }
    println!("-1");
}
