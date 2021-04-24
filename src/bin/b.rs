fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }
    let mut s = 0;
    let mut t = 1000;

    for i in 0..n {
        s = s.max(a[i]);
        t = t.min(b[i]);
    }
    if s <= t {
        println!("{}", t - s + 1)
    } else {
        println!("0")
    }
}
