fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }

    let mut m = 0;
    let mut r = 0;
    for i in 0..n {
        if a[i] > m {
            m = a[i]
        } else {
            r += m - a[i];
        }
    }

    println!("{}", r);
}
