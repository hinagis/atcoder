fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        xy: [(f64, f64); n]
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut f = vec![vec![true; n]; n];
    for i in 0..n {
        f[i][i] = false;
    }
    let mut c = 0;
    for i in 0..n {
        for j in i + 1..n {
            if f[i][j] {
                f[i][j] = false;
                f[j][i] = false;
                let mut r = vec![i, j];
                let mut p = 2;
                if xy[i].0 == xy[j].0 {
                    for k in j + 1..n {
                        if xy[i].0 == xy[k].0 {
                            for &r in &r {
                                f[r][k] = false;
                                f[k][r] = false;
                            }
                            r.push(k);
                            p += 1;
                        }
                    }
                } else {
                    for k in j + 1..n {
                        if xy[k].1 - xy[i].1 == (xy[j].1 - xy[i].1) * (xy[k].0 - xy[i].0) / (xy[j].0 - xy[i].0) {
                            for &r in &r {
                                f[r][k] = false;
                                f[k][r] = false;
                            }
                            r.push(k);
                            p += 1;
                        }
                    }
                }
                if p >= k {
                    c += 1;
                }
            }
        }
    }
    println!("{}", c);
}
