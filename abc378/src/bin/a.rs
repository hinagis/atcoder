fn main() {
    proconio::input! {mut a: [u8; 4]}
    a.sort();
    println!("{}", if a[0] == a[1] {
        if a[2] == a[3] {2} else {1}
    } else {
        if a[1] == a[2] {1} else if a[2] == a[3] {1} else {0}
    });
}
