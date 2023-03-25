fn main() {
    proconio::input! {a: [usize]}
    let mut r = 0;
    let mut c = vec![false; 1000_000_001];
    for &a in &a {
        if c[a] {
            r += 1;
        }
        c[a] = !c[a];
    }
    println!("{}", r);
}
