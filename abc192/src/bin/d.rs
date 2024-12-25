use proconio::{input, marker::Bytes};

fn main() {
    input! {
        x: Bytes,
        m: u64,
    }
    let x = x.iter().map(|d| (d - b'0') as u64).collect::<Vec<_>>();

    if x.len() == 1 {
        println!("{}", if x[0] <= m {'1'} else {'0'});
    } else {
        let b = x.iter().fold(0, |m, &v| v.max(m));
        let calc = |b| x.iter().fold(0u64, |n, &d| n.saturating_mul(b).saturating_add(d));
        let check = |b| calc(b) <= m;
        let binsearch = |mut l, mut r| {
            while l + 1 < r {
                let b = (l + r) / 2;
                if check(b) {
                    l = b;
                } else {
                    r = b;
                }
            }
            l
        };
        println!("{}", binsearch(b, m + 1) - b);
    }
}
