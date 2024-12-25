use proconio::input as I;

fn main() {
    I! {
        n: usize,
        p: [(i64, i64, i64); n],
    }

    let mut d = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            d[i][j] = (p[i].0 - p[j].0).abs() + (p[i].1 - p[j].1).abs();
        }
    }

    let check = |s| {
        for i in 0..n {
            let mut f = vec![false; n];
            f[i] = true;
            let mut q = std::collections::VecDeque::new();
            q.push_back(i);

            while let Some(j) = q.pop_front() {
                for k in 0..n {
                    if ! f[k] && p[j].2 * s >= d[j][k] {
                        f[k] = true;
                        q.push_back(k);
                    }
                }
            }

            if f.iter().all(|&x| x) {
                return true;
            }
        }

        false
    };

    let mut l: i64 = 0;
    let mut r: i64 = u32::max_value() as i64;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if check(m) {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
