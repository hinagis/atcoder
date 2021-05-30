fn main() {
    proconio::input! {
        n: u32,
        k: u32,
    }
    let j = k * (k + 1) / 2;
    let i = n * (n + 1) / 2;
    println!("{}", i * k * 100 + j * n);
}
