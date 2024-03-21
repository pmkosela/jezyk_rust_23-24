fn main() {
    const N: usize = 30;
    let mut remainders: [u32; N] = [0; N];
    for i in 0..30 {
        remainders[i] = 100 % (i + 1) as u32;
    }
    println!("{:?}", remainders);
    for i in 0..30 {
        println!("{}, ", remainders[N - i - 1]);
    }
}
