fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }
    let mut s = vec![0; 2];
    for i in 0..n {
        let u = s[0] + a[i];
        if s[1] > 0 {
            let v = s[1] + a[i] * 2;
            s[0] = s[0].max(v);
        }
        s[1] = s[1].max(u);
    }
    println!("{}", s[0].max(s[1]));
}
