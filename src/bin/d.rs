fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [i64; n + 1],
        mut c: [i64; n + m + 1]
    }

    let mut b = vec![0; m + 1];
    for i in 0..=m {
        b[m - i] = c[n + m - i] / a[n];
        for j in 0..=n {
            c[n + m - (i + j)] -= a[n - j] * b[m - i];
        }
    }
    println!("{}", b.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(" "));
}