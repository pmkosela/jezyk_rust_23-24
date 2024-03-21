fn distance(point1og: (i32, i32), point2og: (i32, i32)) -> f32 {
    let point1 = (point1og.0 as f32, point1og.1 as f32);
    let point2 = (point2og.0 as f32, point2og.1 as f32);
    let square = ((point1.0-point2.0)*(point1.0-point2.0)+
                 ((point1.1-point2.1)*(point1.1-point2.1))) as f32;
    square.sqrt()
}

fn main() {
    println!("{}", distance((0,0),(2,2)));
    return;
    let mut krotka = (7, 10, 5.0);
    println!("{}", krotka.0);
    println!("{}", krotka.1);
    println!("{}", krotka.2);
    krotka.0 = 1;
    println!("{}", krotka.0);

    println!("{:?}", krotka);


    let array = [0, 1, 2, 3];
    for t in array {
        println!("{}", t);
    }
    for i in 0..4 {
        println!("{}", array[i]);
    }
    println!("{:?}", array);
}
