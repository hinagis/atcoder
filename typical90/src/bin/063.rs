fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        p: [[u32; w]; h]
    }

    let mut m = 0;
    for i in 1..1 << h {
        let mut q = vec![];
        for k in 0..h {
            if i & (1 << k) != 0 {
                q.push(k)
            }
        }
        m = m.max({
            let mut m = 0;
            let mut c = std::collections::HashMap::new();
            for j in 0..w {
                let mut r = true;
                for k in 1..q.len() {
                    if p[q[k]][j] != p[q[0]][j] {
                        r = false;
                        break;
                    }
                }
                if r {
                    let p = c.entry(p[q[0]][j]).or_insert(0);
                    *p += 1;
                    m = m.max(*p);
                }
            }
            m * q.len()
        });
    }
    println!("{}", m);
}
