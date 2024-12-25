fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }

    let mut p = 0;
    let mut b = [false; 5];
    for i in 0..n {
        b[0] = true;
        for j in (0..4).rev() {
            if b[j] {
                let c = j + a[i];
                if c < 4 {
                    b[c] = true;
                } else {
                    p += 1;
                }
                b[j] = false;
            }
        }
    }
    println!("{}", p);
}
