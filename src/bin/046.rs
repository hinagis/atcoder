const L: usize = 46;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let mut va = vec![0; L];
    let mut vb = vec![0; L];
    let mut vc = vec![0; L];
    for i in 0..n {
        va[a[i] % L] += 1;
        vb[b[i] % L] += 1;
        vc[c[i] % L] += 1;
    }

    let mut r = 0u64;
    for i in 0..L {
        for j in 0..L {
            for k in 0..L {
                if (i + j + k) % L == 0 {
                    r += va[i] * vb[j] * vc[k];
                }
            }
        }
    }
    println!("{}", r);
}
