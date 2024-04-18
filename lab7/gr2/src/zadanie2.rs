fn position(element: i32, array: &[i32]) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == element {
            return Some(i);
        }
    }
    None
}

fn main() {
    let array = [1, 2, 3, 4];
    let mut position_v = position(2, &array);
    println!("{:?}", position_v);
    position_v = position(7, &array);
    println!("{:?}", position_v);

    let mut index = 0;
    if let Some(t) = position_v {
        index = t;
    } else {
        println!("Elementu nie ma w tablicy");
    }

    println!("Pozycja w tablicy to {}", index);
}
