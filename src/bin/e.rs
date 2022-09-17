use std::io::stdin;

fn main() {
    let n: usize = read().parse().unwrap();
    let mut a = 1;
    let mut b = n + 1;
    while a + 1 != b {
        let m = (a + b) / 2;
        let t = ask(a, m - 1, 1, n);
        if t == m - a {
            a = m;
        } else {
            b = m;
        }
    }
    let mut c = 1;
    let mut d = n + 1;
    while c + 1 != d {
        let m = (c + d) / 2;
        let t = ask(1, n, c, m - 1);
        if t == m - c {
            c = m;
        } else {
            d = m;
        }
    }
    println!("! {} {}", a, c);
}

fn ask(a: usize, b: usize, c: usize, d: usize) -> usize {
    println!("? {} {} {} {}", a, b, c, d);
    read().parse().unwrap()
}

fn read() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.split_whitespace().next().unwrap().to_string()
}
