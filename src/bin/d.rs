fn main() {
    proconio::input! {
        s: [char; 3],
        t: [char; 3]
    }
    let s: String = s.iter().collect();
    let t: String = t.iter().collect();
    macro_rules! P {
        ($s: ident, $t: expr, $f: expr) => {
            match &$s as &str {"RGB" | "GBR" | "BRG" => $t, _ => $f}
        };
    }
    println!("{}", P!(s, P!(t, "Yes", "No"), P!(t, "No", "Yes")));
}
