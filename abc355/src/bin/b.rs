fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [u32; n],
        mut b: [u32; m],
    }
    b.append(&mut a.clone());
    b.sort();
    let mut p = std::collections::HashSet::new();
    for a in a {
        p.insert(a);
    }
    for i in 0..b.len() - 1 {
        if p.contains(&b[i]) && p.contains(&b[i + 1]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
