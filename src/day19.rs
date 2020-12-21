mod day19 {
    use std::collections::HashMap;
    // use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let mut rules: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        let mut is_rule = true;
        let mut inputs: Vec<String> = vec![];
        vec.iter()
            .map(|it| it.trim())
            .map(|it| it.replace("\"", ""))
            .for_each(|it| {
                if it.len() == 0 {
                    is_rule = false;
                    return ();
                }
                if is_rule {
                    let list: Vec<&str> = it.split(":").collect();
                    let v_list: Vec<Vec<String>> = list[1]
                        .split("|")
                        .map(|it| it.trim().split(" ").map(|it| it.to_string()).collect())
                        .collect();

                    rules.insert(list[0].to_string(), v_list);
                } else {
                    inputs.push(it.trim().to_string());
                }
            });
        println!("{:?}", rules);
        println!("{:?}", inputs);
        0
    }

    fn check(rules: HashMap<String, Vec<Vec<String>>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day19_part1() {
        let list = common::parse_from_file("./data/day19_test.txt");
        // let list = common::parse_from_file("./data/day19_part1.txt");
        //     // let v = day19::part1(Box::new(list.unwrap()));
        //     // let v = day19::part1("0,3,6".to_string());
        //     // let v = day19::part1("3,1,2".to_string());
        //     // let v = day19::part1("2,1,3".to_string());
        //     let v = day19::part1(list.unwrap());
        let v = day19::part1(list.unwrap());
        assert_eq!(v, 25972);
    }
}
