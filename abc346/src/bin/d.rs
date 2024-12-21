use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {
        n: usize,
        s: B,
        c: [u64; n]
    }

    let mut l = vec![[0, 0]; n];
    let mut r = vec![[0, 0]; n];
    l[0][(s[0] - b'0') as usize ^ 1] = c[0];
    let j = n - 1;
    r[j][(s[j] - b'0') as usize ^ 1] = c[j];
    for i in 1..n {
        let j = n - 1 - i;
        l[i][0] = l[i - 1][1];
        l[i][1] = l[i - 1][0];
        r[j][0] = r[j + 1][1];
        r[j][1] = r[j + 1][0];
        l[i][(s[i] - b'0') as usize ^ 1] += c[i];
        r[j][(s[j] - b'0') as usize ^ 1] += c[j];
    }

    let mut m = std::u64::MAX;
    for i in 1..n {
        m = m.min(l[i - 1][0] + r[i][0]);
        m = m.min(l[i - 1][1] + r[i][1]);
    }
    println!("{}", m);
}
