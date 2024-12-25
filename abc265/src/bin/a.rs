fn main() {
    proconio::input! {
        x: usize,
        y: usize,
        n: usize,
    }
    println!("{}", if x * 3 <= y {n * x} else {(n % 3) * x + (n / 3) * y});
}
