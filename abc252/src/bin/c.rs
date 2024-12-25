use proconio::{input as I, marker::Bytes as B};
fn main() {
    I! {
        n: usize,
        s: [B; n]
    }
    let s = s.iter().map(|s| s.iter().map(|&c| (c - b'0') as usize).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut r = vec![0; 10];
    for i in 0..10 {
        let mut m = 0;
        let mut c = vec![0; 10];
        for j in 0..n {
            let mut k = 0;
            while k < 9 {
                if s[j][k] == i {
                    break;
                }
                k += 1;
            }
            c[k] += 1;
            if c[k] > c[m] || (c[k] == c[m] && k > m) {
                m = k;
            }
        }
        r[i] = (c[m] - 1) * 10 + m;
    }
    println!("{}", r.iter().fold(usize::max_value(), |m, &v| m.min(v)));
}
