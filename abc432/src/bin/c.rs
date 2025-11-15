use proconio::input as I;

fn main() {
    I! {
        n: usize,
        x: u64,
        y: u64,
        mut a: [u64; n]
    }
    a.sort();
    if a[0] * y < a.last().unwrap() * x {
        println!("-1");
        return;
    }
    let d = y - x;
    let l = a[0] * y;
    let mut c = a.iter().sum::<u64>();
    for i in 1..n {
        let r = a[i] * y;
        if (r - l) % d != 0 {
            println!("-1");
            return;
        }
        c -= (r - l) / d;
    }

    println!("{}", c);
}
