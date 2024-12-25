use std::collections::HashSet;

fn main() {
    proconio::input! {n: u64}
    let mut v = HashSet::new();
    let mut i = 2;
    while i * i <= n {
        let mut j = i * i;
        if !v.contains(&j) {
            while j <= n {
                v.insert(j);
                j *= i;
            }
        }
        i += 1;
    }

    let mut r = 0;
    for _ in &v {
        r += 1;
    }
    println!("{}", n - r);
}
