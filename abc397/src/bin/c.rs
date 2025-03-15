use std::collections::HashMap as H;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let (mut l, mut r) = (H::new(), H::new());
    l.insert(a[0], 1);
    for i in 1..n {
        *r.entry(a[i]).or_insert(0) += 1;
    }
    let mut m = l.len() + r.len();
    for i in 1..n - 1 {
        *l.entry(a[i]).or_insert(0) += 1;
        let e = r.get_mut(&a[i]).unwrap();
        if *e == 1 {
            r.remove(&a[i]);
        } else {
            *e -= 1;
        }
        m = m.max(l.len() + r.len());
    }
    println!("{}", m);
}
