fn main() {
    proconio::input! {
        n: usize,
        w: u32,
        a: [u32; n],
    }

    let mut h = std::collections::HashSet::new();
    for i in 0..n {
        if a[i] <= w {
            h.insert(a[i]);
        }
        for j in i + 1..n {
            if a[i] + a[j] <= w {
                h.insert(a[i] + a[j]);
                for k in j + 1..n {
                    if a[i] + a[j] + a[k] <= w {
                        h.insert(a[i] + a[j] + a[k]);
                    }
                }
            }
        }
    }
    println!("{}", h.len());
}
