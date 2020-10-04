fn rl() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn rt() -> Vec<i32> {
    rl().trim().split_whitespace()
        .map(|e| e.parse().unwrap()).collect()
}

fn count(n: i32, y: i32) -> (i32, i32, i32) {
    let y = y / 1000;
    if y == n {
        return (0, 0, y);
    }

    let (mut n10, mut n5, mut n1) = (0, 0, y);
    while n1 >= 5 && n10 + n5 + n1 > n {
        n1 -= 5;
        n5 = (y - n1) / 5;
        n10 = 0;
        while n5 >= 2 && n10 + n5 + n1 > n {
            n5 -= 2;
            n10 += 1;
        }
    }

    if n10 + n5 + n1 == n {
        return (n10, n5, n1);
    } else {
        return (-1, -1, -1);
    }
}
fn main() {
    let v = rt();
    let (n, y) = (v[0], v[1]);

    let (n10, n5, n1) = count(n, y);
    println!("{} {} {}", n10, n5, n1);
}
