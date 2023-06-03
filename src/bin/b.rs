fn main() {
    proconio::input! {mut n: u32}
    let mut k = 0;
    while n >= 1000 {
        n /= 10;
        k += 1;
    }
    for _ in 0..k {
        n *= 10;
    }
    println!("{}", n);
}
