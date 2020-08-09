fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n],
    }
    let mut r = 0;

    let mut i = 0;
    while i < n {
        if a[i] % 2 == 1 {
            r += 1;
        }
        i += 2;
    }
    println!("{}", r);
}
