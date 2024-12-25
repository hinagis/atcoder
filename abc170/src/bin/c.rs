use proconio::input;

fn main() {
    input! {
        x: usize,
        n: usize,
    }
    if n == 0 {
        println!("{}", x);
        return;
    }

    input! {
        mut p: [usize; n],
    }
    p.sort();
    for i in 0..101 {
        if x >= i && p.contains(&(x - i)) == false {
            println!("{}", x - i);
            return;
        } else {
            if p.contains(&(x + i)) == false {
                println!("{}", x + i);
                return;
            }
        }
    }
}
