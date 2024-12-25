use rand::prelude::*;

fn main() {
    let start_time = get_time();

    let mut rnd: rand::rngs::StdRng = rand::SeedableRng::from_seed([0; 32]);

    proconio::input! {
        d: usize,
        c: [i64; 26],
        s: [[i64; 26]; d],
    }

    let mut t = make_first_t(d, &c, &s);
    let mut sum_v = calc(d, &c, &s, &t).iter().sum::<i64>();

    while get_time() - start_time < 2000_000_000 - 100_000_000 {
        let dqi = (rnd.gen_range(1, d), rnd.gen_range(1, 26));
        let pret = t[dqi.0 - 1];
        t[dqi.0 - 1] = dqi.1;
        let sum_vi = calc(d, &c, &s, &t).iter().sum::<i64>();
        if sum_vi > sum_v {
            sum_v = sum_vi;
        } else {
            t[dqi.0 - 1] = pret;
        }
    }

    for di in 0..d {
        println!("{}", t[di]);
    }
}

fn get_time() -> u64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() * 1000_000_000 + t.subsec_nanos() as u64
}

fn make_first_t(
    d: usize,
    c: &Vec<i64>,
    s: &Vec<Vec<i64>>,
) -> Vec<usize> {
    let mut t = vec![0; 365];
    let mut last_d = vec![0; 26];
    let mut prev = 0;
    for di in 0..d {
        let mut max_v = 0;
        let mut max_i = 0;
        for i in 0..26 {
            let pre_last_di = last_d[i];
            last_d[i] = di + 1;
            let v = sum_di(di, prev, &c, &s, i + 1, &last_d);
            last_d[i] = pre_last_di;
            if v > max_v {
                max_v = v;
                max_i = i;
            }
        }
        prev = max_v;
        last_d[max_i] = di + 1;
        t[di] = max_i + 1;
    }
    t
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

fn sum_di(
    di: usize,
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
