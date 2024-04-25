fn krotsze_niz(wektor: Vec<&str>, dlugosc: usize) -> Vec<&str> {
    wektor.iter().filter(|s| s.len() < dlugosc).map(|x| *x).collect()
}

fn ma_podwojne_litery(s: &str) -> bool {
    // przekomplikowane, do poprawki
    s.chars().reduce(|acc, c| {
    if let Some(a) = acc {
        if acc == c { return None; }
    } else {
        None
    }).is_none()
}

fn z_powtorzeniami(wektor: Vec<&str>) -> Vec<&str> {
    wektor.iter().filter(ma_podwojne_litery).map(|x| *x).collect()
}


fn main() {
    let napisy = vec!["kot", "pies", "mleko", "defenestracja praska",
                      "lekki", "pizza", "kebab"];
    println!("{:?}", krotsze_niz(napisy.clone(), 4));
    println!("{:?}", z_powtorzeniami(napisy.clone()));
}
