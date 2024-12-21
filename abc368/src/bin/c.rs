fn main() {
    proconio::input! {
        n: usize,
        h: [u64; n]
    }
    let mut t = 0;
    for i in 0..n {
        let s = (h[i] / 5) * 3;
        let m = h[i] % 5;
        t += match t % 3 {
            0 => s + if m < 4 {m} else {3},
            1 => s + if m < 3 {m} else {2},
            _ => s + if m < 2 {m} else if m > 3 {2} else {1},
        };
    }

    println!("{}", t);
}
