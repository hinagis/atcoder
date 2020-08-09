fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }

    let mut r = 1000u64;
    for i in 0..n - 1{
        if a[i] < a[i + 1] {
            r += (r / a[i]) * (a[i + 1] - a[i]);
        }
    }
    println!("{}", r);
}
