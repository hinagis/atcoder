fn main() {
    proconio::input! {
        h: u32,
        w: u32,
        a: u32,
        b: u32,
    }

    println!("{}", dfs(h, w, a, b, 0, 0));
}

fn dfs(h: u32, w: u32, a: u32, b: u32, i: u32, bit: u32) -> u32 {
    if i == h * w {
        1
    } else {
        let br = 1 << i;
        if bit & br == br {
            dfs(h, w, a, b, i + 1, bit)
        } else {
            let bl = br << 1;
            let mut r = 0;
            if a > 0 {
                if i % w != w - 1 && !bit & bl == bl {
                    r += dfs(h, w, a - 1, b, i + 1, bit | bl | br)
                }
                if i + w < h * w {
                    let bw = 1 << (i + w);
                    r += dfs(h, w, a - 1, b, i + 1, bit | bw | br)
                }
            }
            if b > 0 {
                r += dfs(h, w, a, b - 1, i + 1, bit | br)
            }
            r
        }
    }
}
