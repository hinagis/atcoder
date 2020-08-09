fn main() {
    proconio::input! {
        l: usize,
        r: usize,
        d: usize,
    }
    let mut s = 0;

    for i in l..=r {
        if i % d == 0 {
            s += 1;
        }
    }
    println!("{}", s);
}
