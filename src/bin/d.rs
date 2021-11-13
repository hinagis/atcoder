fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        mut a: [u64; n]
    }
    a.sort();

    let mut r = 0;
    for i in 0..n - k {
        r += a[i];
    }

    let mut h = std::collections::HashMap::new();
    for i in n - k..n {
        *h.entry(a[i]).or_insert(0) += 1;
    }

    let mut j = 0;
    let mut c = a[n - k];
    while r > 0 {
        let p = a[n - k + j];
        j += h.get(&a[n - k + j]).unwrap();
        if j >= k {
            c += r / k as u64;
            r = 0;
        } else {
            let d = a[n - k + j] - p;
            if r >= j as u64 {
                if r >= j as u64 * d {
                    c += d;
                    r -= j as u64 * d;
                } else {
                    c += r / j as u64;
                    r = 0;
                }
            } else {
                r = 0;
            }
        }
    }
    println!("{}", c);
}
