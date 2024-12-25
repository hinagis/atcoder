fn main() {
    proconio::input! {
        n: usize,
        x: u32,
        vp: [(u32, u32); n]
    }
    let x = x * 100;
    let mut s = 0;
    for i in 0..n {
        s += vp[i].0 * vp[i].1;
        if s > x {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
