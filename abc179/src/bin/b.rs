fn main() {
    proconio::input! {
        n: usize,
        d: [(usize, usize); n],
    }
    for i in 0..(n - 2) {
        if d[i].0 == d[i].1 && d[i + 1].0 == d[i + 1].1 && d[i + 2].0 == d[i + 2].1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
