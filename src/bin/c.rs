fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n]
    }

    let x = (b[0][0] - 1) % 7;
    let y = (b[0][0] - 1) / 7;

    if x + m > 7 {
        println!("No");
        return;
    }

    for i in 0..n {
        for j in 0..m {
            if b[i][j] != ((i + y) * 7) + (j + x + 1) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
