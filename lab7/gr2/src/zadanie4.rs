fn wizytowka_lab(imie: &String, nazwisko: &String) -> String {
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

fn wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String {
    /*let imie_str;
    let nazwisko_str;
    if let Some(s) = imie {
        imie_str = s;
    } else {
        imie_str = String::from("Jan");
    }
    if let Some(s) = nazwisko {
        nazwisko_str = s;
    } else {
        nazwisko_str = String::from("Kowalski");
    }*/

    // -- Alternatywnie
    /*let imie_str;
    let nazwisko_str;
    if imie.is_some() {
        imie_str = imie.unwrap();
    } else {
        imie_str = String::from("Jan");
    }
    if nazwisko.is_some() {
        nazwisko_str = nazwisko.unwrap();
    } else {
        nazwisko_str = String::from("Kowalski");
    }*/

    // -- Najlepiej
    let imie_str = imie.unwrap_or(String::from("Jan"));
    let nazwisko_str = nazwisko.unwrap_or(String::from("Kowalski"));
    wizytowka_lab(&imie_str, &nazwisko_str)
}

fn main() {
    let imie = Some(String::from("Onufry"));
    let nazwisko = None;
    let wizytowka_str = wizytowka(imie, nazwisko);
    println!("{}", wizytowka_str);
}
