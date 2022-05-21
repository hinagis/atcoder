fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [u32; n],
        b: [usize; k]
    }
    let mut h = std::collections::HashSet::new();
    for &b in &b {
        h.insert(b - 1);
    }
    let mut a = a.iter().enumerate().map(|(i, &v)| (v, i)).collect::<Vec<_>>();
    a.sort();
    let v = a[n - 1].0;
    for i in (0..n).rev() {
        if a[i].0 != v {
            break;
        }
        if h.contains(&a[i].1) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
