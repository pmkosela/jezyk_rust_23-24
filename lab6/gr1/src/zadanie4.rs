fn od_tylu(tekst: &String) -> String {
    let iterator: String = tekst.chars().rev().collect();
    iterator
}

fn main() {
    let imie = String::from("onufry");
    let wyjscie = od_tylu(&imie);
    println!("{}", &wyjscie);
}
