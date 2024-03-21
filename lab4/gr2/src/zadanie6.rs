fn swap_range(arr: &mut [u32], r1: (usize, usize), r2: (usize, usize)) {
    let mut range1 = r1;
    let mut range2 = r2;
    // Tutaj obetniemy range, jeżeli któryś jest dłuższy
    let len1 = range1.1 - range1.0;
    let len2 = range2.1 - range2.0;
    if len1 > len2 {
        range1.1 = range1.0 + len2;
    } else if len2 > len1 {
        range2.1 = range2.0 + len1;
    }

    let mut offset = 0;
    for i in range1.0..=range1.1 {
        let tmp = arr[range2.0 + offset];
        arr[range2.0 + offset] = arr[i];
        arr[i] = tmp;
        offset += 1;
    }
}

fn main() {
    let mut array = [0,1,2,3,4,5,6,7,8,9];
    let range1 = (1, 3);
    let range2 = (5, 7);
    let range3 = (5, 8);
    println!("{:?}", array);
    swap_range(&mut array, range1, range2);
    println!("{:?}", array);
    swap_range(&mut array, range1, range3);
    println!("{:?}", array);
}
