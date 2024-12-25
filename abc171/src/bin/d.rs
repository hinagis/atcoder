use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        bc: [(i64, i64); q],
    }

    let mut r = 0;
    let mut h = HashMap::new();
    for aj in a {
        *h.entry(aj).or_insert(0) += 1;
        r += aj;
    }

    for i in 0..q {
        let (b, c) = bc[i];
        let hbv = {
            let hb = h.entry(b).or_insert(0);
            let hbv = *hb;
            *hb = 0;
            hbv
        };
        let hc = h.entry(c).or_insert(0);
        *hc += hbv;

        r += hbv * (c - b);
        println!("{}", r);
    }
}
