fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }

    let mut r = 0;
    for i in 0..n {
        r += a[i];
    }

    println!("Yes");
    println!("{}", r);
}
