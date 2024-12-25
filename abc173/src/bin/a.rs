fn main() {
    proconio::input! {
        n: usize,
    }
    let r = n % 1000;
    let r = if r == 0 { 0 } else { 1000 - r };
    println!("{}", r);
}
