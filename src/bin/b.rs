fn main() {
    proconio::input! {
        n: proconio::marker::Chars,
    }

    let mut r = 0;
    for ni in n {
        r = (r + ni.to_digit(10).unwrap()) % 9;
    }
    dbg!(r);
    println!("{}", if r == 0 {"Yes"} else {"No"});
}
