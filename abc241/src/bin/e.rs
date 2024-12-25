use proconio::input as I;

fn main() {
    I! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut f = vec![(0, !0); n];
    f[0].0 = 0;
    let mut i = 0;
    let mut x = 0;
    while k > 0 {
        let j = x % n;
        if f[j].1 < i {
            let d = i - f[j].1;
            x += (x - f[j].0) * (k / d);
            k %= d;
            if k == 0 {
                break;
            }
        }
        f[j] = (x, i);
        x += a[j];
        i += 1;
        k -= 1;
    }
    println!("{}", x);
}
