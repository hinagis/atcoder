fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        c: [usize; n]
    }
    let mut d = std::collections::HashMap::new();
    for i in 0..k {
        *d.entry(c[i]).or_insert(0) += 1;
    }
    let mut r = d.len();
    for i in k..n {
        *d.entry(c[i]).or_insert(0) += 1;
        if *d.get(&c[i - k]).unwrap() > 1 {
            *d.get_mut(&c[i - k]).unwrap() -= 1;
        } else {
            d.remove(&c[i - k]);
        }
        r = r.max(d.len());
    }

    println!("{}", r);
}
