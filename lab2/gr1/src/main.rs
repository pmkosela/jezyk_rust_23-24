//fn suma_liczb(n: u64) {
fn suma_liczb(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    
    while i < n {
        i += 1;
        if i == 5 {
            //continue;
            //break;
            return 1222;
        }
        sum += i;
    }
    sum
}

fn main() {
    // println!("Hello, world!");
    //
    // let mut sum = 0;
    // let mut i = 0;
    // let n = 10;
    //
    // while i < n {
    //     i += 1;
    //     if i == 5 {
    //         continue;
    //         //break;
    //     }
    //     sum += i;
    // }
    // println!("Suma {} pierwszych liczb naturalnych to {}", n, sum);
    // sum = 0;
    // i = 0;
    //
    // loop {
    //     i += 1;
    //     if i == n {
    //         //continue;
    //         break;
    //     }
    //     sum += i;
    // }
    // println!("Suma {} pierwszych liczb naturalnych to {}", n, sum);
    // sum = 0;
    // i = 0;
    //
    // for j in 0..10 {
    //     sum += j;
    // }
    // println!("Suma {} pierwszych liczb naturalnych to {}", n, sum);

    // let n = 5;
    // let mut silnia = 1;
    // for i in 1..=n{
    //     silnia *= i;
    // }
    // println!("Silnia wynosi {}",silnia);

    // let mut number = 240;
    // let mut suma = 0;
    // while number > 0{
    //     let n = number%10;
    //     suma += n;
    //     number = number/10;
    // }
    // println!("{suma}");

    let n = 15;
    let k: u16 = 8;
    n += k;
    println!("Suma {} to {}", n, suma_liczb(n));

    for i in 1..n{
        for j in i+1..n{
            for k in j+1..n{
                if i*i+j*j==k*k {
                    println!("{} {} {}",i,j,k);
                }
            }
        }
    }
}
