fn main() {
    proconio::input! {
        n: usize,
        p: u32,
        a: [u32; n]
    }
    let mut r = 0;
    for &a in &a {
        if a < p {
            r += 1;
        }
    }

    println!("{}", r);
}
