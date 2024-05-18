fn main() {
    proconio::input! {h: u64}
    let mut r = 0;
    for i in 0.. {
        r += 2u64.pow(i);
        if r > h {
            println!("{}", i + 1);
            return;
        }
    }
}
