fn is_zero(number: i32) -> Result<f32, u8> {
    if number == 0 {
        Err(0)
    } else {
        Ok(number as f32)
    }
}

fn fraction(numerator: i32, denominator: i32) -> Result<f32, u8> {
    /*if denominator == 0 {
        Err(0)
    } else {
        Ok(numerator as f32 / denominator as f32)
    }*/
    let numerator_f = numerator as f32;
    let denominator_f = is_zero(denominator)?;
    Ok(numerator_f / denominator_f)
}

fn main() {
    println!("{:?}", fraction(1, 2));
    println!("{:?}", fraction(1, 0));
}
