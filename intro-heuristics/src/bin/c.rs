fn main() {
    proconio::input! {
        d: usize,
        c: [i64; 26],
        s: [[i64; 26]; d],
        mut t: [usize; d],
        m: usize,
        dq: [(usize, usize); m],
    }

    for i in 0..m {
        t[dq[i].0 - 1] = dq[i].1;
        let v = calc(d, &c, &s, &t);
        println!("{}", v[d - 1]);
    }
}

fn sum_di(di: usize,
    prev: i64,
    c: &Vec<i64>,
    s: &Vec<Vec<i64>>,
    t: usize,
    last_d: &Vec<usize>,
) -> i64 {
    let mut v = prev + s[di][t - 1];
    for i in 0..26 {
        v -= c[i] * ((di + 1 - last_d[i]) as i64);
    }
    v
}

fn calc(d: usize,
    c: &Vec<i64>,
    s: &Vec<Vec<i64>>,
    t: &Vec<usize>,
) -> Vec<i64> {
    let mut last_d = vec![0; 26];

    let mut prev = 0;
    let mut v = vec![0; 365];
    for di in 0..d {
        last_d[t[di] - 1] = di + 1;
        prev = sum_di(di, prev, &c, &s, t[di], &last_d);
        v[di] = prev;
    }
    v
}
