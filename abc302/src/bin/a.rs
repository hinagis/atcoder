fn main() {
    proconio::input! {
        a: u64,
        b: u64
    }
    let c = a / b;

    println!("{}", c + if a == b * c {0} else {1});
}
