fn main() {
    proconio::input! {s: [(i64, i64)]}
    let n = s.len();
    let mut p = s
        .iter()
        .map(|&(l, r)| if l <= 0 && 0 <= r {0} else {if l.abs() < r.abs() {l} else {r}})
        .collect::<Vec<i64>>();
    let mut t: i64 = p.iter().sum();
    for i in 0..n {
        let (l, r) = s[i];
        if t == 0 {
            break;
        } else if t < 0 {
            if r > 0 {
                let d = r - p[i];
                if d < t.abs() {
                    p[i] = r;
                    t += d;
                } else {
                    p[i] += t.abs();
                    t = 0;
                    break;
                }
            }
        } else {
            if l < 0 {
                let d = l - p[i];
                if d.abs() < t {
                    p[i] = l;
                    t += d;
                } else {
                    p[i] -= t;
                    t = 0;
                    break;
                }
            }
        }
    }
    if t == 0 {
        println!("Yes\n{}", p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    } else {
        println!("No");
    }
}
