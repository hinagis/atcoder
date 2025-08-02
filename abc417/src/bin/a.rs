fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
        mut s: String
    }
    println!("{}", s.drain(a..n - b).collect::<String>());
}
