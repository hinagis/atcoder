use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut wh = vec![(n, n - 2)];
    let mut hw = vec![(n, n - 2)];
    let mut r = (n - 2) * (n - 2);
    for _ in 0..q {
        input! {f: usize, x: usize}
        let (wh, hw) = if f == 1 {(&mut wh, &mut hw)} else {(&mut hw, &mut wh)};
        if let Some(w) = wh.last_mut() {
            if x < w.0 {
                let h = hw.last().unwrap().0 - 2;
                r -= h;
                if w.1 == h {
                    (*w).0 = x;
                } else {
                    wh.push((x, h));
                }
            } else {
                let bs = || {
                    let mut l = 0;
                    let mut r = wh.len();
                    while l < r {
                        let m = (l + r) / 2;
                        if x < wh[m].0 {
                            l = m + 1;
                        } else {
                            r = m;
                        }
                    }
                    l
                };
                r -= wh[bs()].1;
            }
        }
    }
    println!("{}", r);
}
