fn main() {
    proconio::input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    }
    let mut p = vec![false; x + 101];
    p[0] = true;
    for &(a, b) in & ab {
        let mut t = vec![false; x + 100];
        for i in 0..x {
            if p[i] {
                t[i + a] = true;
                t[i + b] = true;
            }
        }
        p = t
    }
    println!("{}", if p[x] {"Yes"} else {"No"});
}
