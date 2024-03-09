use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        t: usize
    }
    let mut s = vec![0; n];
    let mut c = std::collections::HashMap::new();
    c.insert(0, n);

    for _ in 0..t {
        I! {
            a: U,
            b: u64
        }

        let p = c.get_mut(&s[a]).unwrap();
        *p -= 1;
        if *p == 0 {
            c.remove(&s[a]);
        }
        s[a] += b;
        *c.entry(s[a]).or_default() += 1;

        println!("{}", c.len());
    }
}
