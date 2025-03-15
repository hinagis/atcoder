fn main() {
    proconio::input! {x: String}
    let (i, j) = x.split_once('.').unwrap();
    let (i, j) = (i.parse::<i32>().unwrap(), j.parse::<i32>().unwrap());
    println!("{}", if i >= 38 {1} else if i >= 37 && j >= 5 {2} else {3});
}
