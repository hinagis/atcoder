use std::collections::HashMap as H;

fn main() {
    proconio::input! {n: u64}
    let mut h = H::new();
    println!("{}", f(&mut h, n));
}

fn f (h: &mut H<u64, u64>, n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        let n2 = if let Some(&n) = h.get(&(n / 2)) {n} else {f(h, n / 2)};
        h.insert(n / 2, n2);
        let n3 = if let Some(&n) = h.get(&(n / 3)) {n} else {f(h, n / 3)};
        h.insert(n / 3, n3);
        n2 + n3
    }
}
