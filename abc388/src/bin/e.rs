fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut c = vec![0; n];
    let mut j = 1;
    for i in 0..n {
        while j < n && a[j] / 2 < a[i] {
            j += 1;
        }
        if j < n {
            c[j] += 1;
        }
    }
    let mut t = 0;
    let mut s = 0;
    for i in 0..n {
        s += c[i];
        if s > 0 {
            t += 1;
            s -= 1;
        }
    }
    let m = t;

    let mut c = vec![0; n];
    let mut j = 1;
    for i in (0..n).rev() {
        while j > 0 && a[j] > a[i] / 2 {
            j -= 1;
        }
        if a[j] <= a[i] / 2 {
            c[j] += 1;
        }
    }
    let mut t = 0;
    let mut s = 0;
    for i in 0..n {
        s += c[i];
        if s > 0 {
            t += 1;
            s -= 1;
        }
    }
    println!("{}", m.min(t));
}
