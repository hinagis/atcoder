fn main() {
    proconio::input! {
        _: u32,
        _: u32,
        n: usize,
        mut p: [(u32, u32); n],
        mut a: [u32],
        mut b: [u32]
    }
    a.sort();
    b.sort();
    p.sort_by(|a, b| a.0.cmp(&b.0));

    let mut c = vec![vec![]; a.len() + 1];
    let mut k = 0;
    for i in 0..a.len() {
        let mut j = k;
        while j < n && p[j].0 < a[i] {
            c[i].push(p[j].1);
            j += 1;
        }
        k = j;
    }
    while k < n {
        c[a.len()].push(p[k].1);
        k += 1;
    }
    for i in 0..a.len() {
        c[i].sort();
    }
    let mut l = std::u32::MAX;
    let mut r = 0;
    for i in 0..a.len() {
        let mut k = 0;
        for u in 0..b.len() {
            let j = match c[i].binary_search(&b[u]) {
                Ok(e) => e - 1,
                Err(e) => e
            };
            let d = (j - k) as u32;
            l = l.min(d);
            r = r.max(d);
            k = j;
        }
        let d = (c[i].len() - k) as u32;
        l = l.min(d);
        r = r.max(d);
    }

    println!("{} {}", l, r);
}
