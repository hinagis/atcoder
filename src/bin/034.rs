fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [u32; n]
    }

    let mut h = std::collections::HashMap::new();
    let mut m = 0;
    let mut c = 0;
    let mut l = 0;
    for r in 0..n {
        let p = {
            let p = h.entry(a[r]).or_insert(0);
            *p += 1;
            *p
        };
        if p == 1 {
            if h.len() > k {
                m = m.max(c);
                while h.len() > k {
                    let d = {
                        let d = h.get_mut(&a[l]).unwrap();
                        *d -= 1;
                        *d
                    };
                    if d == 0 {
                        h.remove(&a[l]);
                    }
                    c -= 1;
                    l += 1;
                }
            }
        }
        c += 1;
    }
    m = m.max(c);

    println!("{}", m);
}
