fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [u64; n],
        k: [u64; q]
    }
    let mut o = vec![];
    let mut d = 0;
    o.push((0, 0));
    if a[0] != 1 {
        o.push((1, 0));
    }
    for i in 0..n-1 {
        d += 1;
        if a[i] + 1 != a[i + 1] {
            o.push((a[i] + 1 - d, d))
        }
    }
    d += 1;
    o.push((a[n - 1] + 1 - d, d));
    for i in 0..q {
        let mut ok = 0;
        let mut ng = o.len();
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if o[mid].0 <= k[i] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", k[i] + o[ok].1);
    }
}
