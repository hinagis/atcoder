fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n]
    }
    let mut f = vec![(false, false); n];
    for i in 1..n - 1 {
        if p[i] > p[i - 1] && p[i] > p[i + 1] {
            f[i].0 = true;
        }
        if p[i] < p[i - 1] && p[i] < p[i + 1] {
            f[i].1 = true;
        }
    }
    let mut c = (
        (if f[1].0 {1} else {0}) + (if f[2].0 {1} else {0}),
        (if f[1].1 {1} else {0}) + (if f[2].1 {1} else {0}),
    );
    let mut t = 0;
    let mut l = 0;
    let mut r = 3;
    loop {
        if r - l < 3 || c.0 == 0 || c.1 == 0 {
            c.0 += if f[r].0 {1} else {0};
            c.1 += if f[r].1 {1} else {0};
            r += 1;
            if r >= n {break}
            continue;
        }
        if p[l] > p[l + 1] || c.0 > 1 || c.1 > 1 {
            l += 1;
            c.0 -= if f[l].0 {1} else {0};
            c.1 -= if f[l].1 {1} else {0};
            continue;
        }

        let (mut u, mut v) = (l + 1, r + 1);
        while r - u >= 3 && !f[u].0 && !f[u].1 {
            u += 1;
        }
        while v < n && !f[v - 1].0 && !f[v - 1].1 {
            v += 1;
        }
        t += (u - l) * (v - r);
        c.0 -= if f[u].0 {1} else {0};
        c.1 -= if f[u].1 {1} else {0};
        l = u;
    }
    println!("{}", t);
}
