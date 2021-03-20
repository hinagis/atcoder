fn main() {
    proconio::input! {x: String}
    println!("{}", x.split('.').collect::<Vec<_>>()[0]);
}
