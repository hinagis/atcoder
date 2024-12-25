fn main() {
    proconio::input! {
        n: usize,
    }
    let n2 = n * n;

    println!("{}", n + n2 + n2 * n);
}
