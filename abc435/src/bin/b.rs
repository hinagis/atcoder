use proconio::input as I;

fn main() {
    I! {
        n: usize,
        a: [u32; n]
    }
    let mut c = 0;
    for l in 0..n {
        let mut s = 0;
        for r in l..n {
            s += a[r];
            c += 1;
            for i in l..=r {
                if s % a[i] == 0 {
                    c -= 1;
                    break;
                }
            }
        }
    }
    println!("{}", c);
}
