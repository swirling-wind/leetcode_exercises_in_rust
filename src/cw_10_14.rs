
#[derive(Debug)]
struct Brain<'life> {
    data_ptr: usize,
    data: Vec<u8>,

    op_ptr: usize,
    operations: &'life [u8],

    input_ptr: usize,

    output: Vec<u8>,
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut br = Brain {
        data_ptr: 0_usize,
        data: vec![0_u8; 100],

        op_ptr: 0_usize,
        operations: code.as_bytes(),

        input_ptr: 0_usize,

        output: vec![],
    };

    loop {
        if br.op_ptr >= br.operations.len() {
            break;
        }
        match br.operations[br.op_ptr] as char {
            '>' => {
                br.data_ptr += 1;
            }
            '<' => {
                br.data_ptr -= 1;
            }
            '+' => {
                br.data[br.data_ptr] = br.data[br.data_ptr].wrapping_add(1);
            }
            '-' => {
                br.data[br.data_ptr] = br.data[br.data_ptr].wrapping_sub(1);
            }
            '.' => {
                br.output.push(br.data[br.data_ptr]);
            }
            ',' => {
                br.data[br.data_ptr] = input[br.input_ptr];
                br.input_ptr += 1;
            }
            '[' => {
                if br.data[br.data_ptr] == 0 {
                    let mut pairs: u32 = 1;
                    loop {
                        br.op_ptr += 1;

                        if br.operations[br.op_ptr] == '[' as u8 {
                            pairs += 1;
                        } else if br.operations[br.op_ptr] == ']' as u8 {
                            pairs -= 1;
                            if pairs == 0 {
                                break;
                            }
                        }
                    }
                    continue;
                }
            }
            ']' => {
                if br.data[br.data_ptr] != 0 {
                    let mut pairs: u32 = 1;
                    loop {
                        br.op_ptr -= 1;

                        if br.operations[br.op_ptr] == ']' as u8 {
                            pairs += 1;
                        } else if br.operations[br.op_ptr] == '[' as u8 {
                            pairs -= 1;
                            if pairs == 0 {
                                break;
                            }
                        }
                    }
                    continue;
                }
            }
            _ => panic!("Unknown instruction: [{}] !", br.operations[br.op_ptr] as char),
        };

        br.op_ptr += 1;
    }

    br.output
}

fn make_readable(seconds: u32) -> String {
    format!("{:02}:{:02}:{:02}",
            seconds / 3600,
            (seconds % 3600) / 60,
            seconds % 60)
}



fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return String::from("now");
    }

    let years = seconds / 31536000;
    let days = (seconds % 31536000) / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let elements = [years, days, hours, minutes, seconds];
    let mut result: String = String::new();

    match years {
        0 => {}
        1 => {
            result.push_str("1 year");
        }
        _ => {
            result.push_str(format!("{} years", years).as_str());
        }
    };

    if elements[0..1].iter().sum::<u64>() > 0 {
        match elements[1..].iter().filter(|&&value| value > 0).count() {
            0=>{},
            1=> {
                result.push_str(" and ");
            },
            _ => {
                result.push_str(", ");
            }
        };
    }

    match days {
        0 => {}
        1 => {
            result.push_str("1 day");
        }
        _ => {
            result.push_str(format!("{} days", days).as_str());
        }
    };

    if elements[0..2].iter().sum::<u64>() > 0 {
        match elements[2..].iter().filter(|&&value| value > 0).count() {
            0=>{},
            1=> {
                result.push_str(" and ");
            },
            _ => {
                result.push_str(", ");
            }
        };
    }

    match hours {
        0 => {}
        1 => {
            result.push_str("1 hour");
        }
        _ => {
            result.push_str(format!("{} hours", hours).as_str());
        }
    };

    if elements[0..3].iter().sum::<u64>() > 0 {
        match elements[3..].iter().filter(|&&value| value > 0).count() {
            0=>{},
            1=> {
                result.push_str(" and ");
            },
            _ => {
                result.push_str(", ");
            }
        };
    }

    match minutes {
        0 => {}
        1 => {
            result.push_str("1 minute");
        }
        _ => {
            result.push_str(format!("{} minutes", minutes).as_str());
        }
    };
    if elements[0..4].iter().sum::<u64>() > 0 {
        match elements[4..].iter().filter(|&&value| value > 0).count() {
            0=>{},
            1=> {
                result.push_str(" and ");
            },
            _ => {
                result.push_str(", ");
            }
        };
    }
    match seconds {
        0 => {}
        1 => {
            result.push_str("1 second");
        }
        _ => {
            result.push_str(format!("{} seconds", seconds).as_str());
        }
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn brain_luck_tests() {
        assert_eq!(String::from_utf8(brain_luck(",+.-.", ez_vec("UFO", 255))).unwrap(), "VU");
        assert_eq!(String::from_utf8(brain_luck(",+.-.,+.-.", ez_vec("UFO", 255))).unwrap(), "VUGF");
        // // Echo until byte 255 encountered
        assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Cod", 255))).unwrap(), "Cod");
        assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
        // Echo until byte 0 encountered
        assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
        // Multiply two numbers
        assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
    }

    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
        let mut v = s.to_string().into_bytes();
        v.push(i);
        v
    }

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    #[ignore]
    fn make_readable_fixed_tests() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(format_duration(15731080), "182 days, 1 hour, 44 minutes and 40 seconds");
    }
}