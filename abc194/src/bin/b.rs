fn main() {
    proconio::input! {
        n: usize,
        ab: [(u32, u32); n]
    }

    let mut t = ab[0].0 + ab[0].1;
    for i in 0..n {
        let a = ab[i].0;
        for j in 0..n {
            let b = ab[j].1;
            t = t.min(if i == j {
                a + b
            } else {
                a.max(b)
            });
        }
    }
    println!("{}", t);
}
