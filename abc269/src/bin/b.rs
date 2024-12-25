fn main() {
    proconio::input! {s: [String; 10]}

    let mut a = 10;
    for i in 0..10 {
        if s[i] != ".........." {
            a = i + 1;
            break;
        }
    }

    let mut b = 10;
    for i in a..10 {
        if s[i] == ".........." {
            b = i;
            break;
        }
    }

    let mut c = 10;
    for (j, s) in s[a - 1].chars().enumerate() {
        if s != '.' {
            c = j + 1;
            break;
        }
    }

    let mut d = 10;
    for (j, s) in s[a - 1].chars().enumerate() {
        if j >= c && s == '.' {
            d = j;
            break;
        }
    }

    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
