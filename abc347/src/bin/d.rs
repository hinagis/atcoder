fn main() {
    proconio::input! {
        a: u128,
        b: u128,
        c: u128,
    }
    let d = c.count_ones() as u128;
    let f = a.min(d);
    for i in 0..=f {
        if a + d - i * 2 == b {
            let mut j = i;
            let mut k = a - i;
            let mut x = 0;
            let mut i = 0;
            while j > 0 || k > 0 {
                if c & (1 << i) == 0 {
                    if k > 0 {
                        x |= 1 << i;
                        k -= 1;
                    }
                } else {
                    if j > 0 {
                        x |= 1 << i;
                        j -= 1;
                    }
                }
                i += 1;
            }
            let y = c ^ x;
            if x < 2u128.pow(60) && y < 2u128.pow(60) {
                println!("{} {}", x, c ^ x);
            } else {
                println!("-1");
            }
            return;
        }
    }

    println!("-1");
}
