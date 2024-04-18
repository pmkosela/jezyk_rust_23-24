fn fraction(numerator: i32, denominator: i32) -> Option<f32> {
    if denominator == 0 {
        None
    } else {
        let numerator = numerator as f32;
        let denominator = denominator as f32;
        let out = numerator / denominator;
        Some(out)
    }
}

fn main() {
    println!("---------");
    println!("{:?}", fraction(1, 2));
    println!("{:?}", fraction(1, 0));

    let wynik = fraction(1, 2);
    println!("{:?}", wynik.unwrap().sqrt());
    let wynik = fraction(1, 0);
    println!("{:?}", wynik.expect("to jest błąd").sqrt());
}
