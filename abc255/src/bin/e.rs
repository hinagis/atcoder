fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        s: [i64; n - 1],
        x: [i64; m]
    }

    let mut c = std::collections::HashMap::new();
    let mut calc = |a: i64, f: i64| {
        for j in 0..m {
            *c.entry(f * (x[j] - a)).or_insert(0) += 1;
        }
    };

    let mut a = 0;
    let mut f = 1;
    calc(a, f);
    for i in 0..n - 1 {
        a = s[i] - a;
        f *= -1;
        calc(a, f);
    }

    println!("{}", c.values().fold(0, |s, &c| s.max(c)));
}
