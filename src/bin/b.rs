fn main() {
    proconio::input! {
        n: usize,
        l: [u64; n],
    }

    let mut r = 0;
    if n >= 3 {
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if l[i] != l[j]
                     && l[j] != l[k]
                     && l[k] != l[i]
                     && l[i] + l[j] > l[k] 
                     && l[j] + l[k] > l[i]
                     && l[k] + l[i] > l[j] {
                        r += 1;
                    }
                }
            }
        }
    }
    println!("{}", r);
}
