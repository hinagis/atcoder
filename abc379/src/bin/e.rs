use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {
        n: usize,
        s: B
    }
    let mut a = s
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * (b - b'0') as usize)
        .collect::<Vec<_>>();
    for i in 1..n {
        a[i] += a[i - 1];
    }

    let mut r = vec![];
    let mut c = 0;
    for i in (0..n).rev() {
        c += a[i];
        r.push(c % 10);
        c /= 10;
    }
    while c > 0 {
        r.push(c % 10);
        c /= 10;
    }
    println!("{}", r
        .iter()
        .rev()
        .map(|c| c.to_string())
        .collect::<String>());
}
