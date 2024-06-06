mod fraction;
use crate::fraction::*;

pub fn add(left: usize, right: usize) -> usize {
    #[cfg(test)]
    println!("to jest test");
    let t = Fraction::new(1, 2);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        //assert_ne!(result, 3);
        //panic!();
        //assert!(true);
    }
}
