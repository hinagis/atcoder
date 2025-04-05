fn main() {
    proconio::input! {
        n: u64,
        m: u32
    }
    let mut s = 1;
    let mut e = 1;
    for _ in 1..=m {
        e *= n;
        s += e;
        if s > 10u64.pow(9) {
            println!("inf");
            return;
        }
    }

    println!("{}", s);
}
