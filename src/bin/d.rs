fn main() {
    proconio::input! {n: u64, a: u64, b: u64}
    let f = |x: u64| x * (x + 1) / 2;
    let c = num::Integer::lcm(&a, &b);
    println!("{}", f(n) - (a * f(n / a) + b * f(n / b) - c * f(n / c)))
}
