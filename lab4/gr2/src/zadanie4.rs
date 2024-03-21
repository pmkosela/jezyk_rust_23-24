fn avg(array: &[u32]) -> f32 {
    let mut sum = 0;
    for i in 0..array.len(){
        sum += array[i];
    }

    sum as f32 / array.len() as f32
}

fn main() {
    let array: [u32; 6] = [1,2,3,4,5,6];
    let avg = avg(&array);
    println!("Średnia {}", avg);
}
