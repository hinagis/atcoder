fn main() {
    proconio::input! {h: u32, w: u32, a: u32, b: u32}
    println!("{}", DFS::new(h, w).dfs(a, b, 0, 0));
}

struct DFS {w: u32, s: u32}
impl DFS {
    fn new(h: u32, w: u32) -> Self {Self {w, s: h * w}}
    fn dfs(self: &Self, a: u32, b: u32, i: u32, bit: u32) -> u32 {
        if i == self.s {
            1
        } else {
            let bi = 1 << i;
            if bit & bi == bi {
                self.dfs(a, b, i + 1, bit)
            } else {
                let bit = bit | bi;
                let mut r = 0;
                if a > 0 {
                    // i isn't right-end, and i's rihgt-side isn't fill.
                    let br = bi << 1;
                    if i % self.w != self.w - 1 && bit & br == 0 {
                        r += self.dfs(a - 1, b, i + 1, bit | br)
                    }
                    // i isn't last-row.
                    if i + self.w < self.s {
                        r += self.dfs(a - 1, b, i + 1, bit | (1 << (i + self.w)))
                    }
                }
                r + if b > 0 {self.dfs(a, b - 1, i + 1, bit)} else {0}
            }
        }
    }
}
