fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        k: u64,
    }
    let mut c = 0;
    while a * k.pow(c) < b {
        c += 1;
    }
    println!("{}", c);
}
