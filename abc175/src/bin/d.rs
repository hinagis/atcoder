use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
        c: [i64; n],
    }

    let mut r = calc(&p, &c, k, p[0]);
    for i in 1..n {
        r = r.max(calc(&p, &c, k, p[i]));
    }
    println!("{}", r);
}

fn calc(p: &Vec<usize>, c: &Vec<i64>, k: usize, i: usize) -> i64 {
    let mut j = p[i];
    let mut cum = (1, c[j]);
    let mut v = cum.1;

    while cum.0 < k && j != i {
        j = p[j];
        cum.0 += 1;
        cum.1 += c[j];
        v = v.max(cum.1);
    }

    if cum.0 < k && cum.1 > 0 {
        let l = k / cum.0;
        if l > 1 {
            cum.0 *= l - 1;
            cum.1 *= l as i64 - 1;
            v = v.max(cum.1);
        }
        while cum.0 < k {
            j = p[j];
            cum.0 += 1;
            cum.1 += c[j];
            v = v.max(cum.1);
        }
    }

    v
}
