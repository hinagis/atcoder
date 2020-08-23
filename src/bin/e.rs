use proconio::{input, marker::Usize1};
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        hw: [(Usize1, Usize1); m]
    }
    let mut sum_h = vec![std::collections::HashSet::new(); h];
    let mut sum_v = vec![0; w];
    let mut max_h = 0;
    let mut max_v = 0;
    for (i, j) in hw {
        sum_h[i].insert(j);
        sum_v[j] += 1;
        max_h = max_h.max(sum_h[i].len());
        max_v = max_v.max(sum_v[j]);
    }

    let mut max_vs = std::collections::HashSet::new();
    for j in 0..w {
        if sum_v[j] >= max_v {
            max_vs.insert(j);
        }
    }

    let mut f = false;
    for i in 0..h {
        if sum_h[i].len() >= max_h {
            for j in &max_vs {
                if !sum_h[i].contains(&j) {
                    f = true;
                    break;
                }
            }
            if f {
                break;
            }
        }
    }

    println!("{}", max_h + max_v - if f { 0 } else { 1 });
}
