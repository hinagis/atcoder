use itertools::Itertools;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h]
    }
    println!("{}", (0..w).map(|j| (0..h).map(|i| a[i][j]).join(" ")).join("\n"));
}
