fn main() {
    proconio::input! {
        _: usize,
        t: String
    }

    let (mut x, mut y) = (0, 0);
    let (mut u, mut v) = (1, 0);
    for c in t.chars() {
        match c {
            'S' => {x += u; y += v}
            _ => {let t = u; u = v; v = -t}
        }
    }
    println!("{} {}", x, y);
}
