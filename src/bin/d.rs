fn main() {
    proconio::input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut h = std::collections::HashSet::new();
    h.insert(0);
    for i in 0..n {
        let (a, b) = ab[i];
        let mut g = h.clone();
        for &e in h.iter() {
            for j in 1..=b {
                let y = e + a * j;
                if y == x {
                    println!("Yes");
                    return;
                } else if y < x {
                    g.insert(y);
                } else {
                }
            }
        }
        h = g;
    }
    println!("No");
}
