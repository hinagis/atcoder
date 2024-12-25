use proconio::{input, marker::Bytes};

fn main() {
    input! {
        k: u32,
        s: Bytes,
        t: Bytes
    }
    let mut sc = vec![0; 10];
    let mut tc = vec![0; 10];
    for i in 0..4 {
        sc[(s[i] - b'0') as usize] += 1;
        tc[(t[i] - b'0') as usize] += 1;
    }
    let mut n = 0f64;
    let mut d = 0f64;
    for i in 1..=9 {
        sc[i] += 1;
        if sc[i] + tc[i] <= k {
            for j in 1..=9 {
                tc[j] += 1;
                if sc[j] + tc[j] <= k {
                    let r = if i == j {
                        (k - (sc[i] + tc[i]) + 2, k - (sc[i] + tc[i]) + 1)
                    } else {
                        (k - (sc[i] + tc[i]) + 1, k - (sc[j] + tc[j]) + 1)
                    };
                    let sv = calc(&sc);
                    let tv = calc(&tc);
                    if sv > tv {
                        n += r.0 as f64 * r.1 as f64;
                    }
                    d += r.0 as f64 * r.1 as f64;
                }
                tc[j] -= 1;
            }
        }
        sc[i] -= 1;
    }
    println!("{}", n / d);
}

fn calc(c: &Vec<u32>) -> u64 {
    let mut s = 0;
    for i in 1..=9 {
        s += i as u64 * 10u64.pow(c[i]);
    }
    s
}