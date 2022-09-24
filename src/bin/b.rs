fn main() {
    proconio::input! {
        x: i32,
        y: i32,
        z: i32,
    }
    println!("{}", if x > 0 {
        if y > 0 && y < x {
            if z < y {
                if z < 0 {
                    x + z.abs() * 2
                } else {
                    x
                }
            } else {
                -1
            }
        } else {
            x
        }
    } else {
        if y < 0 && y > x {
            if z > y {
                if z > 0 {
                    x.abs() + z.abs() * 2
                } else {
                    x.abs()
                }
            } else {
                -1
            }
        } else {
            x.abs()
        }
    });
}
