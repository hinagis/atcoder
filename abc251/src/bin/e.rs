fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }

    let calc = |f: (u64, u64)| {
        let mut d = vec![(0, 0); n];
        d[0] = f;
        for i in 1..n {
            d[i].0 = d[i - 1].1;
            d[i].1 = d[i - 1].0.min(d[i - 1].1) + a[i];
        }
        d[n - 1]
    };
    let m = calc((u64::max_value(), a[0]));

    println!("{}", calc((0, u64::max_value())).1.min(m.0).min(m.1));
}
