fn main() {
    proconio::input! {
        x: u64,
        y: u64
    }
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (x, y) = (y, calc(x,  y));
    dbg!(y);
    let (_, y) = (y, calc(x,  y));

    println!("{}", y);
}

fn calc(x: u64, y: u64) -> u64 {
    let mut a = x + y;
    let mut b = 0;
    while a > 0 {
        b = b * 10 + a % 10;
        a /= 10;
    }
    b
}
