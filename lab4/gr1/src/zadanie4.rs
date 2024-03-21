fn sort(a: &mut u32, b: &mut u32, c: &mut u32) {
    if *a > *b {
        let tmp = *b;
        *b = *a;
        *a = tmp;
    }
    if *b > *c {
        let tmp = *c;
        *c = *b;
        *b = tmp;
    }
    if *a > *b {
        let tmp = *b;
        *b = *a;
        *a = tmp;
    }
}

fn main() {
    let mut a = 7;
    let mut b = 2;
    let mut c = 1;
    sort(&mut a, &mut b, &mut c);
    println!("{}, {}, {}", a, b, c);
}
