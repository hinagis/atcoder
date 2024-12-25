fn main() {
    proconio::input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [u64; n],
    }
    for i in 0..p - 1 {
        print!("{} ", a[i]);
    }
    for i in r - 1..s {
        print!("{} ", a[i]);
    }
    for i in q..r - 1 {
        print!("{} ", a[i]);
    }
    for i in p - 1..q {
        print!("{} ", a[i]);
    }
    for i in s..n {
        print!("{} ", a[i]);
    }
    println!();
}
