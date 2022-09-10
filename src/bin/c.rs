fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n]
    }

    let mut c = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            c[(p[i] + n + j - 1 - i) % n] += 1;
        }
    }
    let mut m = 0;
    for i in 0..n {
        m = m.max(c[i]);
    }
    println!("{}", m);
}
