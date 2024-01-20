use itertools::Itertools;

fn main() {
    proconio::input! {a: [isize]}
    let mut d = std::collections::HashMap::new();
    for (i, &p) in a.iter().enumerate() {
        d.insert(p, i as isize + 1);
    }
    let mut r = vec![];
    let mut p = -1;
    while let Some(&i) = d.get(&p) {
        r.push(i);
        p = i;
    }
    println!("{}", r.iter().join(" "));
}
