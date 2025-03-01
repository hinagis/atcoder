use itertools::Itertools;

fn main() {
    proconio::input! {n: usize}
    let mut b = vec![vec!['#'; n]; n];
    for i in (1..n - 1).step_by(2) {
        for j in i..n - i {
            b[i][j] = '.';
            b[n - i - 1][j] = '.';
            b[j][i] = '.';
            b[j][n - i - 1] = '.';
        }
    }
    println!("{}", b.iter().map(|b| b.iter().join("")).join("\n"));
}
