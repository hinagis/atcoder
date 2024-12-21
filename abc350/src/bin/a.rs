fn main() {
    proconio::input! {s: String}
    let n: u32 = s.chars().skip(3).collect::<String>().parse().unwrap();
    println!("{}", if (n >= 1 && n <= 315) || (n >= 317 && n <= 349) {"Yes"} else {"No"});
}
