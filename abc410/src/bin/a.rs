fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n],
        k: u32
    }
    println!("{}", a.iter().filter(|&&a| k <= a).count());
}
