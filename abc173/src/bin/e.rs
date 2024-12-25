const M: i64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        mut k: usize,
        mut a: [i64; n],
    }

    let mut r = 1;
    if k == n {
        let mut m = false;
        for i in 0..k {
            m ^= a[i] < 0;
            r = r * a[i].abs() % M;
        }
        if m {
            r = M - r;
        }
    } else {
        let mut p = Vec::new();
        let mut z = 0;
        let mut m = Vec::new();

        a.sort();
        for i in 0..n {
            if a[i] > 0 {
                p.push(a[i]);
            } else if a[i] == 0 {
                z += 1;
            } else {
                m.push(a[i].abs());
            }
        }

        let plen = p.len();
        let mlen = m.len();
        if k > plen + mlen {
            r = 0;
        } else {
            if plen == 0 {
                if k > mlen {
                    r = 0;
                } else {
                    if k % 2 == 1 {
                        if z > 0 {
                            r = 0;
                        } else {
                            for i in 0..k {
                                r = r * m[mlen - 1 - i] % M;
                            }
                            r = M - r;
                        }
                    } else {
                        for i in 0..k {
                            r = r * m[i] % M;
                        }
                    }
                }
            } else {
                let plen = if k % 2 == 1 {
                    r = r * p[plen - 1] % M;
                    k -= 1;
                    plen - 1
                } else {
                    plen
                };

                if plen > 0 {
                    let mut pi = plen;
                    if mlen >= 2 {
                        let mut mi = 0;
                        while k >= 2 && pi >= 2 && mi <= mlen - 2 {
                            if m[mi] * m[mi + 1] >= p[pi - 1] * p[pi - 2] {
                                r = r * (m[mi] * m[mi + 1] % M) % M;
                                mi += 2;
                                k -= 2;
                            } else {
                                r = r * p[pi - 1] % M;
                                pi -= 1;
                                k -= 1;
                            }
                        }
                        while k >= 2 && mi <= mlen - 2 {
                            r = r * (m[mi] * m[mi + 1] % M) % M;
                            mi += 2;
                            k -= 2;
                        }
                    }
                    while k > 0 {
                        r = r * p[pi - 1] % M;
                        pi -= 1;
                        k -= 1;
                    }
                } else {
                    for i in 0..k {
                        r = r * m[i] % M;
                    }
                }
            }
        }
    }
    println!("{}", r);
}
