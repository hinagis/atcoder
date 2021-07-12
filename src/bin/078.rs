use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut c = vec![0; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize
        }
        c[a.max(b) - 1] += 1;
    }
    println!("{}", c.iter().filter(|&&c| c == 1).count());
}
