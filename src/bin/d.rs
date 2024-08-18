fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut r = vec![0; 2 * n];
    for i in 0..2 * n - 1 {
        r[i + 1] = (r[i] + a[i % n]) % m;
    }
    let mut b = vec![0u64; m];
    for i in 0..n {
        b[r[i]] += 1;
    }
    let mut s = 0;
    for i in n..2 * n {
        b[r[i - n]] -= 1;
        s += b[r[i]];
        b[r[i]] += 1;
    }
    println!("{}", s);
}
