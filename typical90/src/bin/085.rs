fn main() {
    proconio::input! {k: u64}

    let mut q = vec![];
    let mut i = 1u64;
    while i * i <= k {
        if k % i == 0 {
            q.push(i);
            let j = k / i;
            if j != i {
                q.push(j)
            }
        }
        i += 1;
    }

    let mut r = 0u64;
    for i in 0..q.len() {
        let a = q[i];
        for j in i..q.len() {
            let b = q[j];
            if k / a >= b
            && k % (a * b) == 0
            && b <= k / (a * b) {
                r += 1;
            }
        }
    }

    println!("{}", r);
}
