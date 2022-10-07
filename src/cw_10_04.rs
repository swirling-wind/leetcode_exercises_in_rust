fn high_and_low(numbers: &str) -> String {
    let nums: Vec<i32> = String::from(numbers).split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    let minimum = *(nums.iter().min().unwrap());
    let maximum = *(nums.iter().max().unwrap());
    return maximum.to_string() + " " + &*minimum.to_string();
}

fn validate_pin(pin: &str) -> bool {
    match pin.len() {
        4 | 6 => {
            pin.chars().all(|ch| ch.is_digit(10))
        }
        _ => false,
    }
}

fn longest(a1: &str, a2: &str) -> String {
    let mut ch_vec: Vec<char> = a1.chars().chain(a2.chars()).collect();
    ch_vec.sort_by(|a, b| a.cmp(b));
    ch_vec.dedup();
    ch_vec.into_iter().collect()
    // use std::collections::BTreeSet;
    // format!("{}{}", a1, a2).chars().collect::<BTreeSet<char>>().into_iter().collect()
}

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, bit| ((acc << 1) | bit))

    // let mut result: u32 = 0;
    // for &num in slice {
    //     result =  (result << 1) + num ;
    // }
    // result
}

fn find_missing_letter(chars: &[char]) -> char {
    (chars.windows(2)
        .map(|w| (w[0] as u8, w[1] as u8))
        .find(|&w| w.0 + 1 != w.1)
        .unwrap().0 + 1) as char
    // (chars[0]..=chars[chars.len()-1]).find(|c| !chars.contains(c)).unwrap()
}

fn multiples(mut num: i32) -> i32 {
    // (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()

    // let mut result: i32 = 0;
    // if num >= 3 {
    //     for num in 3..num {
    //         if num % 3 == 0 || num % 5 == 0 {
    //             result += num;
    //         }
    //     }
    // }
    // result

    num -= 1;
    if num <= 3
    { return 0; }
    let floor_3 = num - (num % 3);
    let floor_5 = num - (num % 5);
    let floor_15 = num - (num % 15);
    return (floor_3 * (floor_3 + 3) / 6) + (floor_5 * (floor_5 + 5) / 10) - (floor_15 * (floor_15 + 15) / 30);
}

fn is_valid_ip(ip: &str) -> bool {
    let segments: Vec<&str> = ip.split(".").collect();
    segments.iter().all(|elem| (!elem.starts_with("0") || elem.len() == 1) && elem.parse::<u8>().is_ok())
        && segments.len() == 4

    // let ip = String::from(ip);
    // let parts: Vec<&str> = ip.split(".").collect();
    // if parts.len() != 4 {
    //     return false;
    // }
    // return parts.iter()
    //     .all(|&x|
    //         {
    //             let res = x.parse::<u8>();
    //             match res {
    //                 Ok(num) =>
    //                     {
    //                         if num.to_string().len() == x.len() {
    //                             return true;
    //                         }
    //                        return false;
    //                     }
    //                 Err(_) => false,
    //             }
    //         });
}

fn rot13(message: &str) -> String {
    message.as_bytes().iter().map(|&x| {
        if !('A' as u8..='Z' as u8).contains(&x) && !('a' as u8..='z' as u8).contains(&x) {
            return x as char;
        }
        if ('A' as u8..='M' as u8).contains(&x) || ('a' as u8..='m' as u8).contains(&x) {
            return (x + 13) as char;
        }
        return (x - 13) as char

    }).collect::<Vec<char>>().iter().collect::<String>()
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}

#[cfg(test)]
mod tests {
    use super::validate_pin;

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }

    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
}

#[cfg(test)]
mod longest {
    use super::longest;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }
}

#[cfg(test)]
mod binary_slice_to_number {
    use super::binary_slice_to_number;

    #[test]
    fn example_tests() {
        assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
        assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
        assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
        assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);
    }
}

#[cfg(test)]
mod find_missing_letter {
    use super::find_missing_letter;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}

mod multiples {
    use super::multiples;

    #[test]
    fn sample_tests() {
        // assertion(expected, input);
        assertion(23, 10);
        assertion(33, 11);
        assertion(225, 33);
        assertion(8, 6);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(25719750, 10500);
        assertion(0, 2);
        assertion(0, 3);
    }

    fn assertion(expected: i32, input: i32) {
        let actual = multiples(input);
        assert_eq!(expected, actual, "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input);
    }
}

#[cfg(test)]
mod is_valid_ip {
    use super::is_valid_ip;

    #[test]
    fn sample_test() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));

        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
        assert!(!is_valid_ip("00.139.45.254"));
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod rot13 {
    use super::rot13;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
        dotest("Tes+t", "Grf+g");
        dotest("Test", "Grfg");
    }
}