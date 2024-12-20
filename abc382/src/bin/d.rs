use itertools::Itertools;

fn main() {
    proconio::input! {n: usize, m: usize}
    let mut r = vec![];
    let mut a = (0..n).map(|i| 1 + 10 * i).collect::<Vec<usize>>();
    loop {
        r.push(a.clone());
        let mut  f = true;
        for i in (0..n).rev() {
            a[i] += 1;
            if a[i] <= m - 10 * (n - 1 - i) {
                f = false;
                let b = a[i] - 10 * i;
                for j in i + 1..n {
                    a[j] = b + 10 * j;
                }
                break;
            }
        }
        if f {
            break;
        }
    }
    println!("{}\n{}", r.len(), r.iter().map(|a| a.iter().join(" ")).join("\n"))
}
