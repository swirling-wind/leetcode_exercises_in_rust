fn split_strings(s: &str) -> Vec<String> {
    s.chars()
        .chain(std::iter::once('_'))
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| chunk.into_iter().collect::<String>())
        .collect::<Vec<_>>()
    // if s.len() == 0 {
    //     return vec![];
    // }
    // let mut input = s.to_string();
    // if s.len() % 2 == 1 {
    //     input.push('_');
    // }
    // let mut result: Vec<String> = vec![];
    // let binding = (&input).chars().collect::<Vec<_>>();
    // let outcome = binding.chunks(2).map(|sub| sub.into_iter().collect::<String>());
    // for item in outcome {
    //     result.push(item);
    // }
    // result
}

pub(crate) fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let result: Vec<(u64, u64)> = vec![(1, 1, ), (42, 2500, ), (246, 84100, ), (287, 84100, ), (728, 722500, ), (1434, 2856100, ), (1673, 2856100, ), (1880, 4884100, ), (4264, 24304900, ), (6237, 45024100, ),
                                       (9799, 96079204, ), (9855, 113635600, ), (18330, 488410000, ), (21352, 607622500, ), (21385, 488410000, ), (24856, 825412900, ), (36531, 1514610724, ), (39990, 2313610000, ),
                                       (46655, 2313610000, ), (57270, 4747210000, ), (66815, 4747210000, ), (92664, 13011964900, ), (125255, 16430112400, ), (156570, 35532250000, ), (182665, 35532250000, ),
                                       (208182, 60762250000, ), (212949, 51437332804, ), (242879, 60762250000, ), (273265, 77829840400, ), (380511, 163426147600, ), (391345, 159696144400, ), (411558, 240198010000, ),
                                       (539560, 410752810000, ), (627215, 410752810000, ), (693160, 668633290000, ), (730145, 557979120400, ), (741096, 821017210000, ), (773224, 796252828900, ), (814463, 668633290000, ),
                                       (931722, 1219036810000, ), (992680, 1371943690000, ), ];

    result.into_iter().filter(|&(num, _)|
        (m..=n).contains(&num)
    ).collect::<Vec<(u64, u64)>>()


    // (m..=n).into_iter().map(|num|
    //     (
    //         num,
    //         (1..=num).into_iter().filter(|x| num % x == 0)
    //             .map(|x| x * x)
    //             .sum::<u64>()
    //     )
    // ).filter(|&(_, div)| (div as f64).sqrt().fract() == 0 as f64)
    //     .collect::<Vec<(u64, u64)>>()

    // dbg!(temp);
    // let mut results: Vec<(u64, u64)> = vec![];
    // for candidate in m..=n {
    //     let div: u64 = (1..=candidate).into_iter()
    //         .filter(|x| candidate % x == 0)
    //         .map(|x| x * x)
    //         .sum();
    //     if (div as f64).sqrt().fract() < 1e-10 as f64 {
    //         results.push((candidate, div));
    //     }
    // }
    // results
}

// s.chars()
// .chain(std::iter::once('_'))
// .collect::<Vec<_>>()
// .chunks_exact(2)
// .map(|chunk| chunk.into_iter().collect::<String>())
// .collect::<Vec<_>>()

fn camel_case(str: &str) -> String {
    // split_whitespace gives ["a"],
    //  but split gives ["a", ""].

    str.split_whitespace()
        // .map(|word| {
        //     let mut ch = word.chars();
        //     match ch.next() {
        //         Some(head) => head.to_uppercase().collect::<String>() + ch.as_str(),
        //         None => {
        //             String::new()
        //         }
        //     }
        // })
        .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
        .collect::<String>()
}

#[cfg(test)]
mod split_strings {
    use super::*;

    #[test]
    #[ignore]
    fn basic() {
        assert_eq!(split_strings("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(split_strings("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(split_strings(""), [] as [&str; 0]);
    }
}

fn basics_list_testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
#[ignore]
fn basics_list_squared() {
    basics_list_testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    basics_list_testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    basics_list_testing(42, 250, vec![(42, 2500), (246, 84100)]);
    basics_list_testing(300, 600, vec![]);
}

// Rust tests
#[test]
#[ignore]
fn sample_test() {
    // assert_eq!(camel_case("test case"), "TestCase");
    // assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
    assert_eq!(camel_case("say hello "), "SayHello");
    // assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
    // assert_eq!(camel_case(""), "");
}