fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }
    let n = n.min(8);
    let mut c = vec![None; 200];
    for i in 1..(1 << n) {
        let mut p = 0;
        let mut q = Vec::with_capacity(n);
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                p = (p + a[j]) % 200;
                q.push(j + 1);
            }
        }
        let p = p as usize;
        let q = format!("{} {}", q.len(), q.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
        if let Some(b) = &c[p] {
            println!("Yes");
            println!("{}", b);
            println!("{}", q);
            return;
        } else {
            c[p] = Some(q);
        }
    }
    println!("No");
}
