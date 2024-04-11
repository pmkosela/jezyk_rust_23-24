fn funkcja(tekst: &str) {
    println!("{}", tekst);
}

fn lepsza_funkcja(tekst: &String) {
    println!("{}", tekst);
}

fn main() {
    let mut tekst = "mleko";
    tekst = "mleko kokosowe";
    funkcja(tekst);
    let lepszy_tekst = String::from("lepsze mleko");
    lepsza_funkcja(&lepszy_tekst);
}
