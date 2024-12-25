fn main() {
    proconio::input! {xy: [(i32, i32); 3]}
    let x = if xy[0].0 == xy[1].0 {xy[2].0} else if xy[0].0 == xy[2].0 {xy[1].0} else {xy[0].0};
    let y = if xy[0].1 == xy[1].1 {xy[2].1} else if xy[0].1 == xy[2].1 {xy[1].1} else {xy[0].1};
    println!("{} {}", x, y);
}
