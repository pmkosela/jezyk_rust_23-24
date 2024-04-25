fn kwadratuj(x: u32) -> u32 {
    x*x
}

fn main() {
    //1..=10
    let litery: Vec<_> = ('a'..='z').collect();
    println!("{:?}", litery);

    let kwadraty: Vec<u32> = (1..=10).map(kwadratuj).collect();
    println!("{:?}", kwadraty);
                                            // fn (u32) -> bool
    let podzielne: Vec<u32> = (1..=100).filter(|x| (x % 3 == 0) &&
                                                   (x % 4 != 0))
                                       .collect();
    println!("{:?}", podzielne);
}
