fn main() {
    proconio::input! {
        n: usize,
    }
    let mut r = vec![0u64; n];

    for x in 1.. {
        if x * x >= n {
            break;
        }
        for y in 1.. {
            if x * x + y * y >= n {
                break;
            }
            for z in 1.. {
                if x * x + y * y + z * z + x * y + y * z + z * x > n {
                    break;
                }
                r[x * x + y * y + z * z + x * y + y * z + z * x - 1] += 1;
            }
        }
    }
    for i in 0..n {
        println!("{}", r[i]);
    }
}
