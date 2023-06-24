fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n * 7]
    }
    let mut c = 0;
    let mut i = 0;
    for a in a {
        c += a;
        i += 1;
        if i >= 7 {
            i = 0;
            println!("{}", c);
            c = 0;
        }
    }
    if i != 0 {
        println!("{}", c);
    }
}
