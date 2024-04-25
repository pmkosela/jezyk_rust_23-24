fn indeksy(wektor: Vec<u32>, element: u32) -> Vec<usize> {
    wektor.iter()
          .zip(0..=wektor.len())
          .filter(|(x, _)| **x == element)
          .map(|(_, id)| id)
          .collect()
}

fn krotsze_niz(napisy: Vec<&str>, dlugosc: usize) -> Vec<&str> {
    napisy.iter()
          .filter(|s| s.len() < dlugosc)
          .map(|x| *x)
          .collect()
}

fn main() {
    let napisy = vec!["kot", "pies", "mleko", "aleksander wielki"];
    println!("{:?}", krotsze_niz(napisy.clone(), 4));

    let liczby = vec![1, 2, 3, 3, 4, 5, 2, 1];
    println!("{:?}", indeksy(liczby.clone(), 1));
}
