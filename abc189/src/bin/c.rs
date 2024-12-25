fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut m = n;
    let mut f = vec![false; 100001];
    f[1] = true;
    for i in 0..n {
        if f[a[i]] == false {
            f[a[i]] = true;
            let mut c = 0;
            for j in 0..n {
                if a[j] >= a[i] {
                    c += 1;
                } else {
                    m = m.max(a[i] * c);
                    c = 0;
                }
            }
            m = m.max(a[i] * c);
        }
    }

    println!("{}", m);
}
