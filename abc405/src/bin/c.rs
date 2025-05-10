fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }
    let mut s = 0;
    let mut r = a.iter().sum::<u64>();
    for i in 0..n - 1 {
        r -= a[i];
        s += a[i] * r;
    }
    println!("{}", s);
}
