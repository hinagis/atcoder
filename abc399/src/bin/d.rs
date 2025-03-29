use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            a: [U; 2 * n]
        }
        let mut p = vec![Some(vec![]); n];
        for i in 0..2 * n {
            if (i > 0 && a[i - 1] == a[i])
            || (i < 2 * n - 1 && a[i] == a[i + 1]) {
                p[a[i]] = None;
            }
        }
        let mut c = 0;
        for i in 0..2 * n {
            if let Some(j) = &mut p[a[i]]{
                if j.len() == 0 {
                    if i > 0 {
                        j.push(a[i - 1]);
                    }
                    if i < 2 * n - 1 {
                        j.push(a[i + 1]);
                    }
                    continue;
                }
            }
            if let Some(j) = &p[a[i]] {
                if (i > 0 && p[a[i - 1]] != None && a[i] < a[i - 1] && j.contains(&a[i - 1]))
                || (i < 2 * n - 1 && p[a[i + 1]] != None && a[i] < a[i + 1] && j.contains(&a[i + 1])) {
                    c += 1;
                }
            }
        }
        println!("{}", c);
    }
}
