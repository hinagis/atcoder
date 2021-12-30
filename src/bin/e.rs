fn main() {
    proconio::input! {x: String}
    let x = x.bytes().map(|b| (b - b'0') as u32).collect::<Vec<_>>();
    let mut s = x.iter().fold(0, |s, &x| s + x);
    let mut c = 0;
    let mut y = Vec::with_capacity(x.len());
    for x in x.iter().rev() {
        c += s;
        y.push(((c % 10) as u8 + b'0') as char);
        c /= 10;
        s -= x;
    }
    while c > 0 {
        y.push(((c % 10) as u8 + b'0') as char);
        c /= 10;
    }

    println!("{}", y.iter().rev().collect::<String>());
}
