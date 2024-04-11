fn co_drugi(tekst: &String) -> String {
    let mut output = String::new();
    let mut iterator = tekst.chars().step_by(2);
    for znak in &mut iterator {
        output.push_str(znak.to_string().as_str());
        //iterator.skip(1); -- nie działa
    }
    output
}

fn main() {
    let imie = String::from("onufry");
    let wyjscie = co_drugi(&imie);
    println!("{}", &wyjscie);
}
