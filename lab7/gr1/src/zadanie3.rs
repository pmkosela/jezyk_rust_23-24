fn divisors(number: Option<u32>) -> usize {
    let value = number.unwrap();
    // praca domowa :)
    0
}

fn main() {
    let num = 7;
    println!("{}", divisors(Some(num)));
    println!("{}", divisors(None));
}
