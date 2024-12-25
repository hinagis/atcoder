fn main() {
    proconio::input! {
        a: f64,
        b: f64,
    }
    let calc = |g: f64| a / (g + 1f64).sqrt() + b * g;

    const N: usize = 5;
    let mut l = 0f64;
    let mut r = 10_000_000_000_000_000_000f64;
    let mut v = vec![0f64; N];
    while l.ceil() != r.ceil() {
        let p = (r - l) / N as f64;
        for i in 0..N {
            v[i] = l + i as f64 * p;
        }
        let m = (0..N).fold(0, |m, i| if calc(v[m]) <= calc(v[i]) {m} else {i});
        if m == 0 {
            r = v[1];
        } else if m == N - 1 {
            l = v[N - 2];
        } else {
            l = v[m - 1];
            r = v[m + 1];
        }
    }
    println!("{}", calc(l.ceil() as f64));
}
