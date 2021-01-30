fn main() {
    proconio::input! {mut n: u64}
    while n % 2 == 0 {n /= 2}
    let s = (n as f64).sqrt() as u64;
    let mut r = 0;
    for i in 1..=s {
        if n % i == 0 {r += 2}
    }
    if s * s == n {r -= 1}
    println!("{}", r * 2)
}
