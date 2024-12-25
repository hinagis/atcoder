use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut s = 0;
    for ai in &a {
        s ^= ai;
    }

    for ai in &a {
        println!("{}", s ^ ai);
    }
}
