fn rl() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn rt() -> Vec<i32> {
    rl().trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect()
}

fn main() {
    let mut result = true;
    let n: i32 = rl().trim().parse().unwrap();
    let (mut t, mut x, mut y) = (0, 0, 0);
    for _ in 0..n {
        let v = rt();
        let (nt, nx, ny) =(v[0], v[1], v[2]);
        let dt = nt - t;
        let d = (nx - x).abs() + (ny - y).abs();
        if d > dt || (dt - d) % 2 == 1 {
            result = false
        }
        t = nt; x = nx; y = ny;
    }
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
