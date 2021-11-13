fn main() {
    proconio::input! {
        n: usize,
        s: [u32; n]
    }

    let mut h = std::collections::HashSet::new();
    let mut a = 1;
    while 4 * a * a + 3 * a + 3 * a <= 1000 {
        let mut b = a;
        while 4 * a * b + 3 * a + 3 * b <= 1000 {
            h.insert(4 * a * b + 3 * a + 3 * b);
            b += 1;
        }
        a += 1;
    }

    let mut c = 0;
    for &s in &s {
        if ! h.contains(&s) {
            c += 1;
        }
    }
    println!("{}", c);
}
