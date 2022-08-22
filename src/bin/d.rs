use proconio::input as I;

fn main() {
    I! {
        n: usize,
        p: u64, q: u64, r: u64,
    }

    let mut h = std::collections::HashSet::with_capacity(n);
    h.insert(0);
    let mut s = 0;
    for _ in 0..n {
        I! {a: u64}
        s += a;
        h.insert(s);
    }

    for &x in &h {
        let y = x + p;
        let z = y + q;
        let w = z + r;
        if h.contains(&y) && h.contains(&z) && h.contains(&w) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
