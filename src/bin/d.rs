fn main() {
    proconio::input! {
        n: usize,
        st: [(String, String); n]
    }
    let mut h = std::collections::HashMap::new();
    let mut d = Vec::with_capacity(n);
    for (i, (s, t)) in st.iter().enumerate() {
        h.insert(s, (i, t));
        d.push((false, s));
    }
    for i in 0..n {
        if ! d[i].0 {
            d[i].0 = true;
            let mut f = vec![false; n];
            f[i] = true;
            let mut s = h.get(d[i].1).unwrap();
            while h.contains_key(s.1) {
                s = h.get(s.1).unwrap();
                if f[s.0] {
                    println!("No");
                    return;
                }
                d[s.0].0 = true;
                f[s.0] = true;
            }
        }
    }
    println!("Yes");
}
