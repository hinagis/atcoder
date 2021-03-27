fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }

    if n == 1 {
        println!("{}", a[0]);
    } else {
        fn xor(a: &Vec<u32>, n: usize, p: u32, i: usize) -> u32 {
            if i >= n {
                p
            } else {
                let mut b = 0xffffffff;
                let mut q = 0;
                for j in i..n {
                    q |= a[j];
                    b = b.min(xor(a, n, p ^ q, j + 1));
                }
                b
            }
        }
    
        let mut r = 0xffffffff;
        let mut p = 0;
        for i in 0..(n - 1) {
            p |= a[i];
            r = r.min(xor(&a, n, p, i + 1));
        }
        println!("{}", r);
    }
}
