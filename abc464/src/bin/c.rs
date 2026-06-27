use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize, m: usize,
        adb: [(usize, usize, usize); n]
    }

    let mut h = std::collections::HashMap::new();
    for i in 0..n {
        let (a, _, _) = adb[i];
        *h.entry(a).or_insert(0) += 1;
    }

    let mut q = vec![vec![]; m + 1];
    for i in 0..n {
        let (a, d, b) = adb[i];
        q[d].push((a, b));
    }
    for i in 0..m {
        for &(a, b) in q[i + 1].iter() {
            if a == b {continue}
            let p = *h.get(&a).unwrap();
            if p > 1 {
                *h.get_mut(&a).unwrap() -= 1;
            } else {
                h.remove(&a);
            }
            *h.entry(b).or_insert(0) += 1
        }
        println!("{}", h.len());
    }
}
