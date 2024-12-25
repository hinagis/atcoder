fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut r = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if x == i * 500 + j * 100 + k * 50 {
                    r += 1;
                }
            }
        }
    }
    println!("{}", r);
}
