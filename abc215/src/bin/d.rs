use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }
    let mut e = vec![true; 100001];
    for i in 0..n {
        let v = pfact(a[i]);
        for &j in &v {
            if e[j] {
                for k in (j..100001).step_by(j) {
                    e[k] = false;
                }
            }
        }
    }
    let mut c = 0;
    for i in 1..=m {
        if e[i] {
            c += 1;
        }
    }
    println!("{}", c);
    for i in 1..=m {
        if e[i] {
            println!("{}", i);
        }
    }
}

fn pfact(mut m: usize) -> Vec<usize> {
    let mut v = vec![];
    let mut i = 2;
    while i * i <= m {
        while m % i == 0 {
            m /= i;
            v.push(i);
        }
        i += 1;
    }
    if m != 1 {
        v.push(m)
    }
    v
}
