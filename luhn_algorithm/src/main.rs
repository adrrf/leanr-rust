#![allow(unused_variables, dead_code)]

/*

    Ignore all spaces. Reject number with less than two digits.

    Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.

    After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.

    Sum all the undoubled and doubled digits.

    The credit card number is valid if the sum ends with 0.

*/

pub fn luhn(cc_number: &str) -> bool {
    let mut digits = 0;
    let mut i = 1;
    let mut res = 0;
    
    for ch in cc_number.chars().rev() {
        if ch == ' ' {
            continue;
        } else {
            if ch.is_numeric() {
                if i % 2 == 0 {
                    let double_digit = ch.to_digit(10).unwrap() * 2;
                    let d = double_digit / 10;
                    let dd = double_digit % 10;
                    res += d + dd;
                } else {
                    res += ch.to_digit(10).unwrap();
                }
                digits += 1;
                i += 1;
            } else {
                return false;
            }
        }
    }
    
    if digits < 2 {
        return false;
    } else {
        return res % 10 == 0;
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}