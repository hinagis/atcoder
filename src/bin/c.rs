fn main() {
    proconio::input! {
        n: usize,
        m: u64,
        mut a: [u64; n]
    }
    a.sort();
    let mut c = 1;
    let mut r = 0;
    let mut x = 0;
    for l in 0..n {
        c -= 1;
        while r < n && a[r] < a[l] + m {
            c += 1;
            r += 1;
        }
        x = x.max(c);
    }
    println!("{}", x);
}
