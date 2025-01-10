pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn adder(x: i32, y : i32) -> i32 {
    x + y 
}


pub fn single_digit_adder(x: i8, y: i8) -> i8 {
    fn is_single_digit(x: i8) -> bool {
        x < 10 && x > -10
    }

    if !(is_single_digit(x)) || !(is_single_digit(y)) {
        panic!("Only single digit integers are allowed!");
    } else {
        x + y
    }
}

#[cfg(test)]
mod tests {
    use crate::single_digit_adder;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_dont_work() {
        let result = add(4, 5);
        assert_ne!(result, 9)
    }

    #[test]
    fn it_adds() {
        let sum = single_digit_adder(4, 5);
        assert_eq!(sum, 9);
    }

    #[test]
    #[should_panic]
    fn it_should_only_accept_single_digits() {
        single_digit_adder(11, 4);
    }


}






