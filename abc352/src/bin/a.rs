fn main() {
    proconio::input! {
        _: i32,
        x: i32,
        y: i32,
        z: i32,
    }
    let (x, y) = if x < y {(x, y)} else {(y, x)};
    println!("{}", if x <= z && z <= y {"Yes"} else {"No"});
}
