fn main() {
    proconio::input! {
        n: usize,
        s: [u32; n],
        mut t: [u32; n]
    }
    let mut r = t.clone();
    let mut m = 0;
    let mut i = 0;
    while m < n {
        let j = (i + 1) % n;
        if r[i] + s[i] < r[j] {
            r[j] = r[i] + s[i];
            m = 0;
        } else {
            m += 1;
        }
        i = j;
    }
    println!("{}", r.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("\n"));
}
