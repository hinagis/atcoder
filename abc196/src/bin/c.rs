fn main() {
    proconio::input! {n: u64}

    let mut s = 1;
    let mut t = 10;
    while s * t + s <= n {
        s += 1;
        if s >= t {
            t *= 10;
        }
    }
    println!("{}", s - 1);
}
