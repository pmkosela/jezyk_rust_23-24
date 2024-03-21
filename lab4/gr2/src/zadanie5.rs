fn sort(a: &mut u32, b: &mut u32, c: &mut u32) {
    if *b < *a {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }
    if *c < *b {
        let tmp = *c;
        *c = *b;
        *b = tmp;
    }
    if *b < *a {
        let tmp = *b;
        *b = *a;
        *a = tmp;
    }
}

fn main() {
    let mut a = 7;
    let mut b = 2;
    let mut c = 3;
    sort(&mut a, &mut b, &mut c);
    println!("{}, {}, {}", a, b, c);
}
