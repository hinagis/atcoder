fn main() {
    proconio::input! {
        mut abc: [usize; 3],
    }
    abc.sort();
    println!("{}", abc[1]);
}
