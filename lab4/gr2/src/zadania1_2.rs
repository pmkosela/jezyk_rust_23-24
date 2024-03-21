//fn d2(A: (i32, i32), B: (i32, i32)) -> (u32, u32) {
fn d2(A: (i32, i32), B: (i32, i32)) -> f32 {
    let diff1 = (A.0 - B.0) as f32;
    let diff2 = (A.1 - B.1) as f32;
    let sum = diff1*diff1 + diff2*diff2;
    sum.sqrt()
}

fn d3(A: (i32, i32, i32), B: (i32, i32, i32)) -> f32 {
    let diff1 = (A.0 - B.0) as f32;
    let diff2 = (A.1 - B.1) as f32;
    let diff3 = (A.2 - B.2) as f32;
    let sum = diff1*diff1 + diff2*diff2 + diff3*diff3;
    sum.sqrt()
}

fn main() {
    let mut krotka: (i32, f32, u32) = (10, 2.0, 3);
    krotka.0 = 7;
    krotka = (2, 2.0, 2);
    println!("dist: {}", d2((0,0), (1, 1)));
    println!("dist: {}", d3((0,0,0), (1, 1, 1)));
}
