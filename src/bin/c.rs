fn main() {
    proconio::input! {h: [u32; 3], w: [u32; 3]}

    let mut r = 0;
    for a in 1..=30 {
        if a >= h[0] || a >= w[0] {break;}
        for b in 1..=30 {
            if a + b >= h[0] || b >= w[1] {break;}
            let mut v = [0; 3];
            v[0] = h[0] - (a + b);
            for c in 1..=30 {
                if c >= h[1] || a + c >= w[0] {break;}
                let mut e = [0; 2];
                e[0] = w[0] - (a + c);
                for d in 1..=30 {
                    if c + d >= h[1] || b + d >= w[1] {break;}
                    v[1] = h[1] - (c + d);
                    e[1] = w[1] - (b + d);
                    if e[0] + e[1] < h[2] {
                        v[2] = h[2] - (e[0] + e[1]);
                        if v.iter().fold(0, |s, &v| s + v) == w[2] {
                            r += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", r);
}
