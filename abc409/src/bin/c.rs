fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        d: [usize; n - 1]
    }
    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut h = std::collections::HashMap::new();
    let mut p = 0;
    h.insert(0, 1u64);
    for &d in &d {
        p = (p + d) % l;
        *h.entry(p).or_insert(0) += 1;
    }
    let mut m = 0;
    for i  in 0..l {
        if let Some(a) = h.get(&i) {
            if let Some(b) = h.get(&(i + l / 3)) {
                if let Some(c) = h.get(&(i + 2 * l / 3)) {
                    m += a * b * c;
                }
            }
        }
    }

    println!("{}", m);
}
