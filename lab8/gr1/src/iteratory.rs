fn kwadrat(x: u32) -> u32 {
    x*x
}

fn podzielnosc(x: u32) -> bool {
    (x % 3 == 0) && (x % 4 != 0)
}

fn main() {
    //0..9
    let alfabet: Vec<char> = ('a'..='z').collect();
    println!("{:?}", alfabet);

    //                                  fn (u32) -> u32
    //let kwadraty: Vec<u32> = (1..=10).map(|x| {x*x}).collect();
    let kwadraty: Vec<u32> = (1..=10).map(kwadrat).collect();
    println!("{:?}", kwadraty);
                                            // fn(u32) -> bool
    let podzielne: Vec<u32> = (1..=100).filter(|x| (x % 3 == 0) &&
                                                   (x % 4 != 0))
                                       .collect();
    //let podzielne: Vec<u32> = (1..=100).filter(podzielnosc).collect();
    println!("{:?}", podzielne);
}
