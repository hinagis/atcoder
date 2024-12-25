fn main() {
    proconio::input! {
        n: u32,
        mut m: [u32; 3],
    }
    m.sort();
    let mut s = n;
    for c in (0..=n / m[2]).rev() {
        let r = n - c * m[2];
        for b in (0..=r / m[1]).rev() {
            let r = r - b * m[1];
            if r % m[0] == 0 {
                s = s.min(r / m[0] + b + c);
            }
        }
    }
    println!("{}", s);
}
