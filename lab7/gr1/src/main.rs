fn is_this_zero(number: i32) -> Result<f32, String> {
    if number == 0 {
        Err(String::from("to jest zero"))
    } else {
        Ok(number as f32)
    }
}

//fn fraction(numerator: i32, denominator: i32) -> Result<f32, u8> {
fn fraction(numerator: i32, denominator: i32) -> Result<f32, String> {
    /*if denominator == 0 {
        Err(String::from("dzielenie przez zero"))
    } else {
        Ok(numerator as f32 / denominator as f32)
    }*/

    let denominator_f32 = is_this_zero(denominator)?;
    Ok(numerator as f32 / denominator_f32)
}

fn main() {
    println!("{:?}", fraction(1, 2));
    println!("{:?}", fraction(1, 0));
}
