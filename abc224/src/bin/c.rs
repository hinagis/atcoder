fn main() {
    proconio::input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut c = 0;
    for i in 0..n {
        for j in i + 1..n {
            let (dx1, dy1) = (xy[j].0 - xy[i].0, xy[j].1 - xy[i].1);
            for k in j + 1..n {
                let (dx2, dy2) = (xy[k].0 - xy[i].0, xy[k].1 - xy[i].1);
                if dx1 * dy2 != dx2 * dy1 {
                    c += 1;
                }
            }
        }
    }

    println!("{}", c);
}
