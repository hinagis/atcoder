fn main() {
    proconio::input! {k: u64}
    println!("{}", format!("{:b}", k).chars().map(|c| if c == '1' {'2'} else {'0'}).collect::<String>());
}
