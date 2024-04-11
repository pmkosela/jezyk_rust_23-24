fn co_drugi(tekst: &String) -> String {
    let iterator = tekst.chars().step_by(2);
    let mut out = String::new();
    for znak in iterator {
        out.push_str(znak.to_string().as_str());
        //iterator.skip(1);
    }
    out
}

fn main() {
    let imie = String::from("ilacz");
    let tekst = co_drugi(&imie);
    println!("{}", tekst);
}
