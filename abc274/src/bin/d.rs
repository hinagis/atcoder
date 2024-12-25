fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        y: i32,
        a: [i32; n]
    }

    let mut b = Vec::<i32>::with_capacity(n);
    for i in (2..n).step_by(2) {
        b.push(a[i]);
    }

    let mut h = std::collections::HashSet::new();
    h.insert(a[0]);
    for &e in &b {
        let mut t = std::collections::HashSet::new();
        for &f in &h {
            t.insert(f + e);
            t.insert(f - e);
        }
        h = t;
    }

    if ! h.contains(&x) {
        println!("No");
        return;
    }

    let mut c = Vec::<i32>::with_capacity(n);
    for i in (1..n).step_by(2) {
        c.push(a[i]);
    }

    let mut h = std::collections::HashSet::new();
    h.insert(0i32);
    for &e in &c {
        let mut t = std::collections::HashSet::new();
        for &f in &h {
            t.insert(f + e);
            t.insert(f - e);
        }
        h = t;
    }

    if h.contains(&y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
