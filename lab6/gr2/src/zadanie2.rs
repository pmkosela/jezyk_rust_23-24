fn wizytowka(imie: &String, nazwisko: &String) -> String {
    // J. Kowalski
    let mut out = String::new();

    // imie -> uppercase
    let imie_upper = imie.to_uppercase(); // == ""

    // wyciągamy 1szą literę imienia
    let pierwsza_litera = imie_upper.chars().nth(0).unwrap();
    out.push_str(pierwsza_litera.to_string().as_str());

    // dopisujemy ". "
    out.push_str(". ");

    // nazwisko -> uppercase
    let nazwisko_upper = nazwisko.to_uppercase();
    // wyciągamy 1szą literę
    let pierwsza_litera = nazwisko_upper.chars().nth(0).unwrap();
    out.push_str(pierwsza_litera.to_string().as_str());

    // nazwisko -> lowercase
    let nazwisko_lower = nazwisko.to_lowercase();

    // wyciągamy pozostałe litery
    let mut i = 1;
    while i < nazwisko_lower.len() {
        let litera = nazwisko_lower.chars().nth(i).unwrap();
        out.push_str(litera.to_string().as_str());
        i += 1;
    }

    out
}

fn main() {
    let imie = String::from("sliwka");
    let nazwisko = String::from("KOSELA");
    let zlozenie = wizytowka(&imie, &nazwisko);
    println!("{}", zlozenie);
}
