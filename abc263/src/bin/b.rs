fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n - 1],
    }
    let mut c = 0;
    let mut i = n - 1;
    while i > 0 {
        i = p[i - 1] - 1;
        c += 1;
    }

    println!("{}", c);
}
