fn main() {
    println!("Hello, world!");
    println!("drukujemy linię");
    let mut zmienna = 100;
    //let zmienna = 100;
    //println!("zmienna = {zmienna}");
    /*println!("zmienna = {}", zmienna);*/
    println!("zmienna = {zmienna}");
    println!("zmienna = {}", zmienna);
    //let zmienna = 5;
    //zmienna = 7;
    //zmienna += 3;
    let druga_zmienna = 2;
    /*
     * a + b
     * a - b
     * a * b
     * a / b
     * a % b
     */
    println!("nowa zmienna = {}", zmienna);



    let a = 2;
    let b = 0;
    let c: u8 = 5;
    let c: u16 = 5;
    let c: u32 = 5;
    let c: u64 = 5;
    let c: u128 = 5;
    
    let c: i8 = 5;
    let c: i16 = 5;
    let c: i32 = 5;
    let c: i64 = 5;
    let c: i128 = 5;

    let c: bool = true;
    let c: bool = false;

    let c: f32 = 5.0;
    let c: f64 = 5.0;

    let d: u16 = 5;
    let e: u32 = 5;

    let c: usize = 10;

    println!("a + b = {}", a + b);
    //println!("a * c = {}", a as f64 * c);
    //println!("d + e = {}", d as u32 + e);

    let a = 10;
    let b = 5;
    //if a {
    if a == b && a > b || a < b {
        println!("a - b = {}", a - b);
    } else if a >= b {
        println!("b - a = {}", b - a);
    }

}
