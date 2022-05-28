fn main() {
    proconio::input! {mut v: [u32; 3]}
    let b = v[1];
    v.sort();

    println!("{}", if b == v[1] {"Yes"} else {"No"});
}
