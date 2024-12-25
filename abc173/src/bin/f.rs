fn main() {
    proconio::input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }

    let mut r = n * (n + 1) * (n + 2) / 6;
    for (u, v) in uv {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        r -= u * (n - v + 1);
    }

    println!("{}", r);
}
