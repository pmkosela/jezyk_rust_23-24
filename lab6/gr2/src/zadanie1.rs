fn count_char(napis: String, szukany_znak: char) -> u32 {
    let mut out = 0;
    //let x = napis[3]; -- nie działa
    for znak in napis.chars() {
        if znak == szukany_znak {
            out += 1;
        }
    }
    out
}

fn main() {
    let napis: String = String::from("mleko kokosowe");
    let count_o = count_char(napis, 'o');
    println!("{}", count_o);
}
