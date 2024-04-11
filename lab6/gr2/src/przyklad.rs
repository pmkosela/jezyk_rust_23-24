fn funkcja(napis: &String) {
    println!("{}", napis);
}

fn main() {
    let mut napis = "mleko";
    napis = "kokos";

    let lepszy_napis: String = String::from("lepszy_kokos");

    funkcja(&lepszy_napis);
    //funkcja(&lepszy_napis);
}
