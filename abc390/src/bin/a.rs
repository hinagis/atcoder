fn main() {
    proconio::input! {a: [u8; 5]}
    println!("{}", if
           a[0] == 1 && (a[1] == 2 && (a[2] == 3 && a[3] == 5
                                    || a[2] == 4 && a[3] == 3)
                      || a[1] == 3 &&  a[2] == 2 && a[3] == 4)
        || a[0] == 2 &&  a[1] == 1 &&  a[2] == 3 && a[3] == 4 {
        "Yes"
    } else {
        "No"
    });
}
