fn dist( arr:&[i32],arr1:&[i32] ) ->f32{
let x_d:f32 = (arr[0] - arr1[0]) as f32;
let y_d:f32 = (arr[1] - arr1[1]) as f32;
let z_d:f32 = (arr[2] - arr1[2]) as f32;
let distance:f32 = ((x_d.powi(2)) + (y_d.powi(2)) + (z_d.powi(2))).sqrt();
distance
}
fn main() {
    let arr = [1,2,2];
    let arr1 = [2,2,3];
    let distance  = dist(&arr,&arr1);
    println!("distance {}",distance);
}
