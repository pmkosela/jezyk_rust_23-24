//fn factorial(n: u64) {
fn calc_factorial(n: u64) -> u64 {
    let mut factorial = 1;
    //let n = 5;
    let mut i = 1;
    if n == 6 {
        return 1200;
    }
    while i <= n {
        factorial *= i;
        i += 1;
    }
    //return factorial;
    factorial
}

fn digits_of_number(number: usize) {
    let mut num = number;
    while num != 0 {
        println!("{}", num % 10);
        num /= 10;
    }
} 

fn main() {
    let mut factorial = 1;
    let n = 5;

    let mut i = 1;
    while i <= n {
        factorial *= i;
        i += 1;
    }
    println!("{}! = {}", n, factorial);
    
    let mut j = 0;
    loop {
        if j > 9 {
            break;
        }

        j += 1;
        if j == 7 {
            continue;
        }
        println!("{}", j);
    }

    for t in 0..10 {
        println!("trzecia pętla: {}", t);
    }

    digits_of_number(150);
    let m = 20;
    for a in 1..m{
        for b in a+1..m{
            for c in b+1..m{
                if (a*a)+(b*b)==(c*c){
                    println!("{a},{b},{c}")
                }
            }
        }
    }

    let n = 6;
    println!("{}! = {}", n, calc_factorial(n));
}

   
