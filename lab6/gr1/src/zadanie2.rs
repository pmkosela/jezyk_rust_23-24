fn wizytowka(imie: &String, nazwisko: &String) -> String {
    // Chcemy dostać: "O. Kosela"
    let mut output = String::new();

    // imie -> upper case
    let imie = imie.to_uppercase();

    // wyciagnac 1szą literę z imienia
    let pierwsza_litera = imie.chars().nth(0).unwrap();
    output.push_str(pierwsza_litera.to_string().as_str());

    // dopisać kropkę
    output.push_str(". ");

    // nazwisko -> upper case
    // wyciągnąć 1szą literę z nazwiska
    output.push_str(nazwisko.to_uppercase()
                            .chars()
                            .nth(0)
                            .unwrap()
                            .to_string()
                            .as_str());

    // nazwisko -> lower case
    // wyciągnąć pozostałe litery nazwiska
    let mut i = 1;
    while i < nazwisko.len() {
            output.push_str(nazwisko.to_lowercase()
                                    .chars()
                                    .nth(i)
                                    .unwrap()
                                    .to_string()
                                    .as_str());
            i += 1;
    }
    output
}

fn main() {
    let imie = String::from("onufry");
    let nazwisko = String::from("KOSELA");
    let zlozone = wizytowka(&imie, &nazwisko);
    println!("{}", zlozone);
}
