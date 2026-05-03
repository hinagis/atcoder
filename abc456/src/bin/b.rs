use proconio::input as I;

fn main() {
    I! {a: [[u32; 6]; 3]}
    let mut c = 0;
    for i in 0..6 {
        if a[0][i] < 4 {continue}
        for j in 0..6 {
            if a[1][j] < 4 {continue}
            for k in 0..6 {
                match (a[0][i], a[1][j], a[2][k]) {
                    (4, 5, 6) | (4, 6, 5) |
                    (5, 4, 6) | (5, 6, 4) |
                    (6, 4, 5) | (6, 5, 4) => c += 1,
                    _ => continue
                }
            }
        }
    }
    println!("{:.10}", c as f64 / 216f64);
}
