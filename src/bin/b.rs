fn main() {
    proconio::input! {
        n: usize,
        xy: [(f32, f32); n]
    }
    let mut m = 0f32;
    for i in 0..n {
        let (x, y) = xy[i];
        for j in i + 1..n {
            let (u, v) = xy[j];
            m = m.max(((x - u).powf(2f32) + (y - v).powf(2f32)).sqrt())
        }
    }

    println!("{}", m);
}
