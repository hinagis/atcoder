const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }
    let mut s = 0;
    for i in 1..n {
        s = (s + a[i]) % M;
    }

    let mut r = 0;
    for i in 0..(n - 1) {
        r = (r + (a[i] * s % M)) % M;
        if s < a[i + 1] {
            s += M;
        }
        s -= a[i + 1];
    }

    println!("{}", r);
}
