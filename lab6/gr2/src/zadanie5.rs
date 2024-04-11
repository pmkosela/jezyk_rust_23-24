fn szyfruj(tekst: &String, klucz: usize) -> String {
    let mut acc = String::new();
    let it = tekst.chars();
    let mut offset: usize = 0;
    loop {
        if offset * klucz > tekst.len() {
            break;
        }
        acc += it.clone()
                 .skip(offset * klucz)
                 .take(klucz)
                 .collect::<String>()
                 .chars()
                 .rev()
                 .collect::<String>()
                 .as_str();
        offset += 1;
    }
    acc
}

fn main() {
    let imie = String::from("Aladyn");
    let tekst = szyfruj(&imie, 2);
    println!("{}", tekst);
}
