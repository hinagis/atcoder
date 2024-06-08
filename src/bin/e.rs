use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        a: [U; n]
    }
    let mut f = vec![0; n];
    for i in 0..n {
        calc(&a, &mut f, i);
    }
    println!("{}", f.iter().sum::<u64>());
}

fn calc(a: &Vec<usize>, f: &mut Vec<u64>, i: usize) {

}
