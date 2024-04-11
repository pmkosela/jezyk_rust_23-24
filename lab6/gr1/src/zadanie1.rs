fn count_char(tekst: &String, szukany_znak: char) -> u32 {
    let mut out = 0;
    for znak in tekst.chars() {
        if znak == szukany_znak {
            out += 1;
        }
    }
    //tekst[i] = 'c'; -- nie działa
    return out;
}

fn main() {
    let napis = "mleko kokosowe";
    //let string = String::from("str");
    let string = String::from(napis);
    //let string = String::new();
    let liczba_o = count_char(&string, 'o');
    println!("liczba o: {}", liczba_o);
}
