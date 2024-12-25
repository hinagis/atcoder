fn main() {
    proconio::input! {x: i64}
    let n = x.to_string().len() as i64;
    for n in n..=18 {
        for f in 1..=9 {
            for s in -9..9 {
                let l = f + s * (n - 1);
                if l >= 0 && l <= 9 {
                    let mut r = 0i64;
                    for i in 0..n {
                        r = r * 10 + f + s * i;
                    }
                    if r >= x {
                        println!("{}", r);
                        return;
                    }
                }
            }
        }
    }
}
