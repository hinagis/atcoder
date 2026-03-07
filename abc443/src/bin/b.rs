use proconio::input as I;

fn main() {
    I! {
        n: u32,
        k: u32,
    }
    let mut s = 0;
    for i in 0.. {
        s += n + i;
        if s >= k {
            println!("{}", i);
            break;
        }
    }
}
