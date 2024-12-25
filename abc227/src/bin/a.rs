fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: usize,
    }
    let a = (k + a - 1) % n;

    println!("{}", if a == 0 {n} else {a});
}
