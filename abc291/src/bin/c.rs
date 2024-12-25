fn main() {
    proconio::input! {
        _: usize,
        s: String
    }
    let mut p = (0, 0);
    let mut h = std::collections::HashSet::new();
    h.insert(p);
    for c in s.chars() {
        p = match c {
            'R' => (p.0 + 1, p.1),
            'L' => (p.0 - 1, p.1),
            'U' => (p.0, p.1 + 1),
            'D' => (p.0, p.1 - 1),
            _ => (p.0, p.1)
        };
        if ! h.insert(p) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
