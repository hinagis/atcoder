use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n]
    }

    let mut h = HashSet::new();
    for &(x, y) in &xy {
        h.insert((x, y));
    }

    let mut c = 0;
    for &(x, y) in &xy {
        for &(u, v) in &xy {
            if x != u && y != v {
                if h.contains(&(x, v)) && h.contains(&(u, y)) {
                    c += 1;
                }
            }
        }
    }

    println!("{}", c / 4);
}
