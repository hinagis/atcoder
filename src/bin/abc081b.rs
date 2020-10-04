fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut r = 0;
    while check(&mut a) {
        r += 1;
    }

    println!("{}", r);
}

fn check(a: &mut Vec<usize>) -> bool {
    for a in a {
        if *a % 2 == 0 {
            *a /= 2;
        } else {
            return false;
        }
    }
    true
}
