fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [u64; m],
        x: [[u64; m]; n]
    }
    let mut t = vec![0; m];
    for x in x {
        for i in 0..m {
            t[i] += x[i];
        }
    }
    for i in 0..m {
        if t[i] < a[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
