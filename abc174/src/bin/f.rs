fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        mut c: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut h = std::collections::HashMap::new();
    for i in 0..n {
        (*h.entry(c[i]).or_insert((i + 1, i + 1))).1 = i + 1;
    }
    for (l, r) in lr {
        let mut s = 0;
        for hi in &h {
            if (hi.1).0 >= l && (hi.1).1 <= r {
                s += 1;
            }
        }
        println!("{}", s);
    }
}
