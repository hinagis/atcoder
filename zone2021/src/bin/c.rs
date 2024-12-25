fn main() {
    proconio::input! {
        n: usize,
        p: [[u64; 5]; n]
    }
    let check = |x| {
        let mut s = std::collections::HashSet::new();
        for i in 0..n {
            let mut b = 0u8;
            for j in 0..5 {
                if p[i][j] >= x {
                    b |= 1 << j;
                }
            }
            s.insert(b);
        }
        for x in &s {
            for y in &s {
                for z in &s {
                    if x | y | z == 31 {
                        return true;
                    }
                }
            }
        }
        false
    };
    let mut l = 0;
    let mut r = 1000_000_001;
    while r - l > 1 {
        let c = (l + r) / 2;
        if check(c) {
            l = c
        } else {
            r = c
        }
    }

    println!("{}", l);
}
