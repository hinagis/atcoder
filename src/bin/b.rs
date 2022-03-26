fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut f = [true; 2001];
    for &a in &a {
        f[a] = false;
    }
    for i in 0..=2000 {
        if f[i] {
            println!("{}", i);
            return;
        }
    }
}
