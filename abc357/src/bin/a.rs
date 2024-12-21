fn main() {
    proconio::input! {
        n: usize,
        mut m: usize,
        h: [usize; n],
    }
    for i in 0..n {
        if h[i] > m {
            println!("{}", i);
            return;
        }
        m -= h[i];
    }
    println!("{}", n);
}
