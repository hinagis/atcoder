fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    }
    let mut c = 0;
    let mut d = 0;
    for i in 0..n {
        if a[i] == b[i] {
            c += 1;
        } else {
            if b.contains(&a[i]) {
                d += 1;
            }
        }
    }

    println!("{}\n{}", c, d);
}
