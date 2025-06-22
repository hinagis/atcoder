use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        x: [usize; q]
    }
    let mut p = vec![1; q];
    let mut c = vec![0; n];
    for i in 0..q {
        let k = if x[i] == 0 {
            let mut k = 0;
            for j in 1..n {
                if c[j] < c[k] {
                    k = j;
                }
            }
            k
        } else {
            x[i] - 1
        };
        p[i] = k + 1;
        c[k] += 1;
    }
    println!("{}", p.iter().join(" "));
}
