fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n]
    }
    let p = p.iter().map(|p| p.iter().fold(0, |s, p| s + p)).collect::<Vec<_>>();
    let mut q = p.clone();
    q.sort();
    let mut r = vec![0; 1201];
    let mut s = 1201;
    for i in (0..n).rev() {
        let t = q[i];
        for j in t..s {
            r[j] = n - i;
        }
        s = t;
    }

    for i in 0..n {
        if r[p[i] + 300] <= k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
