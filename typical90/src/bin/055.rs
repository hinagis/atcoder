fn main() {
    proconio::input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n]
    }

    let mut r = 0;
    for i1 in 0..n {
        let s1 = a[i1] % p;
        for i2 in i1 + 1..n {
            let s2 = s1 * a[i2] % p;
            for i3 in i2 + 1..n {
                let s3 = s2 * a[i3] % p;
                for i4 in i3 + 1..n {
                    let s4 = s3 * a[i4] % p;
                    for i5 in i4 + 1..n {
                        let s5 = s4 * a[i5] % p;
                        if s5 == q {
                            r += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", r);
}
