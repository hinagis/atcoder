fn main() {
    proconio::input! {
        n: usize,
        m: [(usize, usize)]
    }
    let mut h = std::collections::HashSet::new();
    for (a, b) in m {
        h.insert((a, b));
        let c = [a > 1, a > 2, a < n, a + 1 < n];
        let d = [b > 1, b > 2, b < n, b + 1 < n];
        if c[0] && d[1] {h.insert((a - 1, b - 2));}
        if c[0] && d[3] {h.insert((a - 1, b + 2));}
        if c[1] && d[0] {h.insert((a - 2, b - 1));}
        if c[1] && d[2] {h.insert((a - 2, b + 1));}
        if c[2] && d[1] {h.insert((a + 1, b - 2));}
        if c[2] && d[3] {h.insert((a + 1, b + 2));}
        if c[3] && d[0] {h.insert((a + 2, b - 1));}
        if c[3] && d[2] {h.insert((a + 2, b + 1));}
    }
println!("{}", n * n - h.len());
}
