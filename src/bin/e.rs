use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        x: u64,
        w: [u64; n],
    }

    let mut s = 0;
    let mut t = 0;
    let mut h = std::collections::HashMap::new();
    let mut v = vec![];
    let mut i = 0;
    let mut c = 0;
    loop {
        t += w[i];
        i = (i + 1) % n;
        c += 1;
        if t >= x {
            v.push(c);
            c = 0;
            h.insert(s, v.len() - 1);
            s = i;
            t = 0;
            if let Some(&t) = h.get(&s) {
                s = t;
                break;
            }
        }
    }
    let e = v.len();
    dbg!(&v, &s, &e);
    for _ in 0..q {
        I! {k: u64}
        if k - 1 < e as u64 {
            println!("{}", v[k as usize - 1]);
        } else {
            let k = s + ((k - 1 - s as u64) % (e - s) as u64) as usize;
            println!("{}", v[k]);
        }
    }
}
