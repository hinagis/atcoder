fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let mut d = 0;
    for i in 0..60 {
        if c & (1 << i) != 0 {
            d += 1;
        }
    }

    println!("{}");
}
