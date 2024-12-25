fn main() {
    proconio::input! {p: [(usize, usize, usize, usize)]}
    let mut f = vec![vec![false; 101]; 101];
    for (a, b, c, d) in p {
        for x in a..b {
            for y in c..d {
                f[x][y] = true;
            }
        }
    }
    println!("{}", f.iter().fold(0, |s, e| s + e.iter().filter(|&&g| g).count()));
}
