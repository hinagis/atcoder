fn main() {
    proconio::input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    for i in 0..(n - 2) {
        let (xi, yi) = xy[i];
        for j in (i + 1)..n {
            let (xj, yj) = xy[j];
            for k in (j + 1)..n {
                let (xk, yk) = xy[k];
                if (yk - yi) * (xj - xi) == (yj- yi) * (xk - xi) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
