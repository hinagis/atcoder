fn main() {
    proconio::input! {x: String}
    let x = x.split(".").collect::<Vec<_>>();
    let (d, p): (u32, u8) = (x[0].parse().unwrap(), x[1].chars().next().unwrap() as u8 - b'0');

    println!("{}", d + if p >= 5 {1} else {0});
}
