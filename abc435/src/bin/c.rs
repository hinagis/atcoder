use proconio::input as I;

fn main() {
    I! {
        n: usize,
        a: [usize; n]
    }
    let mut c = 0;
    let mut l = 0;
    let mut r = 1;
    while l < r.min(n) {
        c += 1;
        r = r.max(l + a[l]);
        l += 1;
    }
    println!("{}", c);
}
