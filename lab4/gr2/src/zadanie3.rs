fn zadanie3() {
    const N: usize = 30;
    let mut remainders: [u32; N] = [0; N];
    for i in 1..=N {
        remainders[i - 1] = 100 % i as u32;
    }
    //println!("{:?}", remainders);
    for i in 1..=N {
        println!("{}", remainders[N - i]);
    }
}

fn main() {
    let array: [u32; 3] = [0, 1, 2];
    for i in array {
        println!("{}", i);
    }
    for i in 0..3 {
        println!("{}", array[i]);
    }
    println!("dlugosc tablicy: {}", array.len());
    println!("");
    zadanie3();
}
