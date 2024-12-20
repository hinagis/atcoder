fn main() {
    proconio::input! {
        n: usize,
        q: [(char, usize)],
    }
    let mut c = 0;
    let (mut l, mut r) = (1, 2);
    for (h, t) in q {
        if h == 'L' {
            c += m(n, l, r, t);
            l = t;
        } else {
            c += m(n, r, l, t);
            r = t;
        }
    }
    println!("{}", c);
}

fn m(n: usize, h: usize, o: usize, t: usize) -> usize {
    if h < t {
        if h < o {
            if o < t {
                h + n - t
            } else {
                t - h
            }
        } else {
            t - h
        }
    } else {
        if o < h {
            if t < o {
                n - h + t
            } else {
                h - t
            }
        } else {
            h - t
        }
    }
}
