fn divisors(number: Option<u32>) -> usize {
    // praca domowa
    let value = number.unwrap();
    0
}

fn main() {
    let number = 7;
    println!("{}", divisors(Some(number)));
    println!("{}", divisors(None));
}
