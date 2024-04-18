fn position(element: i32, array: &[i32]) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == element {
            return Some(i);
        }
    }
    None
}

fn main() {
    let array = [0, 1, 2, 3, 4];
    let element = position(2, &array);
    println!("{:?}", element);

    //if element.is_some() {
    let mut index = 0;
    if element.is_none() {
        println!("Elementu nie ma w tablicy");
    } else {
        println!("Element jest w tablicy");
        //index = element.unwrap();
    }

    if let Some(v) = element {
        index = v;
    }

    if let None = element {
        println!("Elementu nie ma w tablicy");
    }

    println!("Indeks elementu to: {}", index);
}
