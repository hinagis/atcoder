fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [i64; n]
    }

    let mut c = vec![0; n + 1];
    for i in 1..=n {
        c[i] = c[i - 1] + a[i - 1];
    }

    let mut s = 0;
    for i in 0..m {
        s += c[m] - c[i]
    }

    let mut r = s;
    for i in m..n {
        s += m as i64 * a[i];
        s -= c[i] - c[i - m];
        r = r.max(s);
    }

    println!("{}", r);
}
