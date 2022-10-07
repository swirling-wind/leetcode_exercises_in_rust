use std::fmt::format;

fn digital_root(mut n: i64) -> i64 {
    let mut result: i64 = 0;
    loop {
        while n > 0 {
            result += n % 10;
            n /= 10;
        }
        if result >= 10 {
            n = result;
            result = 0;
        } else {
            break;
        }
    }
    result
}

fn int32_to_ip(int: u32) -> String {
    int.to_be_bytes().map(|num| num.to_string()).join(".")

    // [
    //     (int & 0xFF_00_00_00) >> 24,
    //     (int & 0x00_FF_00_00) >> 16,
    //     (int & 0x00_00_FF_00) >> 8,
    //     int & 0x00_00_00_FF,
    // ].map(|num| num.to_string()).join(".")
}

fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut remain = n;
    let result: Vec<i64> = vec![];
    while remain != 0 {

    }


    None
}

#[cfg(test)]
mod digital_root_tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
    }
}

#[cfg(test)]
mod int32_to_ip_tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}

fn decompose_testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {

    decompose_testing(50, Some(vec![1, 3, 5, 8, 49]));
    decompose_testing(44, Some(vec![2, 3, 5, 7, 43]));
    decompose_testing(625, Some(vec![2, 5, 8, 34, 624]));
    decompose_testing(5, Some(vec![3, 4]));

}