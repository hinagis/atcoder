fn main() {
    proconio::input! {
        mut n: u64,
        k: u32,
    }

    for _ in 0..k {
        n = if n % 200 == 0 {
            n / 200
        } else {
            1000 * n + 200
        };
    }
    println!("{}", n);
}
