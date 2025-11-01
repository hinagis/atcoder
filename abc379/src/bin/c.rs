use proconio::input as I;

fn main() {
    I! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m]
    }
    let mut b = x.iter().zip(a.iter()).collect::<Vec<_>>();
    b.sort_by(|&a, &b| a.0.cmp(b.0));

    let mut p = 0;
    let mut c = 0;
    for i in 0..m {
        let x = *b[i].0 - 1;
        let d = x - p;
        p = x;
        if d > c {
            println!("-1");
            return;
        }
        c -= d;
        c += *b[i].1;
    }
    if n - p != c {
        println!("-1");
        return;
    }

    let mut s = n * (n + 1) / 2;
    for i in 0..m {
        s -= x[i] * a[i];
    }
    println!("{}", s);
}
