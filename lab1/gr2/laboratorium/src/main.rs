fn main() {
    println!("Hello, world!");
    //let a;
    let mut a = 7;
    //println!("a = {a}");
    //println!("a = {}", a);
    //a = 10;
    println!("a = {}", a);
    /*
     * a + b
     * a - b
     * a * b
     * a / b
     * a % b
     */

    let a = 5.0;
    let b = 0.0;
    println!("a + b = {}", a + b);

    /*let c: u8 = 5;
    le t c: u16 = 5;
    let c: u32 = 5;
    let c: u64 = 5;
    let c: u128 = 5;
    
    let c: i8 = 5;
    let c: i16 = 5;
    let c: i32 = 5;
    let c: i64 = 5;
    let c: i128 = 5;

    let c: f32 = 5.0;
    let c: f64 = 5.0;

    let c: bool = true;
    let c: char = 'a';
    let c: usize = 10;*/

    let a = 5;
    let b = 5;
    println!("a + b = {}", a as u32 + b);
    let f: f32 = 5.0;
    println!("a + f = {}", a as f32 + f);

    if a > b || b == a {
        println!("a - b = {}", a - b);
    } else if a < b {
        println!("b - a = {}", b - a);
    } else {
        println!("przeciwny wypadek");
    }
}
