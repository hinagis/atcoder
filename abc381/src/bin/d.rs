fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    let mut m = 0;
    let mut h = std::collections::HashMap::new();
    let mut l = 0;
    let mut r = 0;
    while r + 1 < n {
        if a[r] == a[r + 1] {
            if let Some(i) = h.insert(a[r], r) {
                m = m.max(r - l);
                while l < i {
                    h.remove(&a[l]);
                    l += 2;
                }
                l += 2;
            }
            r += 2;
            if r < n && a[r] == a[r - 1] {
                m = m.max(r - l);
                h.clear();
                r -= 1;
                l = r;
            }
        } else {
            m = m.max(r - l);
            h.clear();
            r += 1;
            l = r;
        }
    }
    println!("{}", m.max(r - l));
}
