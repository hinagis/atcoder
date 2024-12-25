use proconio::{input as I};

fn main() {
    I! {
        n: usize,
        k: usize,
    }
    let mut h = std::collections::HashMap::new();
    for _ in 0..n {
        I! {a: usize}
        *h.entry(a).or_insert(0) += 1;
    }
    for i in 0..k {
        if let Some(v) = h.get_mut(&i) {
            *v -= 1;
            if *v == 0 {
                h.remove(&i);
            }
        } else {
            println!("{}", i);
            return;
        }
    }
    println!("{}", k);
}
