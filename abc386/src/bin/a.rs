fn main() {
    proconio::input! {mut c: [u8; 4]}
    c.sort();
    println!("{}", if c[0] == c[1] && c[1] == c[2] && c[2] != c[3]
                   || c[0] != c[1] && c[1] == c[2] && c[2] == c[3]
                   || c[0] == c[1] && c[1] != c[2] && c[2] == c[3] {
        "Yes"
    } else {
        "No"
    });
}
