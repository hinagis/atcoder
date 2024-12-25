fn main() {
    proconio::input! {
        l: usize,
        n: usize,
        m: usize,
        v: [(usize, usize); n],
        u: [(usize, usize); m],
    }
    let mut r = 0;
    let mut i = 0;
    let mut j = 0;
    let mut x = 0;
    let mut c = 0;
    let mut d = 0;
    while x < l {
        let e = v[i].1 - c;
        let f = u[j].1 - d;
        let g = e.min(f);
        x += g;
        if v[i].0 == u[j].0 {
            r += g;
        }
        if e == g {
            i += 1;
            c = 0;
        } else {
            c += g;
        }
        if f == g {
            j += 1;
            d = 0;
        } else {
            d += g;
        }
    }
    println!("{}", r);
}
