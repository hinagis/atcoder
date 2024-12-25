fn main() {
    proconio::input! {n: usize}

    let mut c = vec![0; n];
    for i in 1..n {
        let mut d = Vec::new();
        for j in 1..=(f64::sqrt(i as f64) + 1e-9) as usize {
            if i % j == 0 {
                d.push(j);
                if j != i / j {
                    d.push(i / j);
                }
            }
        }
        c[i] = d.len();
    }

    let mut r = 0;
    for i in 1..n {
        r += c[i] * c[n - i];
    }
    println!("{}", r);
}
