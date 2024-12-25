fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }

    if a > d || b < c {
        println!("No");
    } else {
        println!("Yes");
    }
}
