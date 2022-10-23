fn decompose(n: i64) -> Option<Vec<i64>> {

    fn inner_decompose(mut remains: i32, next_sqrt: i32)-> Option<Vec<i64>> {
        let mut trial_sqrt = next_sqrt;
        let mut temp_result: Option<Vec<i64>> = None;
        loop {
            if remains == 0 {
                return temp_result;
            }
            if trial_sqrt == 0 {
                return None;
            }

            let trial_result = inner_decompose(remains - trial_sqrt.pow(2), trial_sqrt - 1);
            match trial_result {
                Some(inner_result)=> {
                    return Some(inner_result);
                },
                None=> {}
            }
            (todo!());
            trial_sqrt -= 1;
        }
    }


    let mut result: Vec<i64> = vec![];
    let mut current: i64 = n - 1;
    let mut remains = n.pow(2);
    loop {
        if remains == 0 {
            return Some(result.into_iter().rev().collect());
        }
        if current == 0 {
            return None;
        }

        current -= 1;
    }
}


fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
#[ignore]
fn tests_decompose() {
    
    testing(50, Some(vec![1,3,5,8,49]));
    testing(44, Some(vec![2,3,5,7,43]));
    testing(625, Some(vec![2,5,8,34,624]));
    testing(5, Some(vec![3,4]));
    
}