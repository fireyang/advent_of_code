mod day15 {
    use std::collections::HashMap;
    // use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn part1(s: String, turn_size: usize) -> i32 {
        let v: Vec<i32> = s
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut age_map: HashMap<i32, i32> = HashMap::new();
        let mut next_spoken = 0;
        let mut spoken = 0;
        for i in 0..v.len() {
            spoken = v[i];
            age_map.insert(spoken, (i + 1) as i32);
            // println!("spoken:{:?} - {:?}", i + 1, spoken);
        }
        for i in v.len()..turn_size {
            spoken = next_spoken;
            if let Some(age) = age_map.get(&spoken) {
                next_spoken = (i + 1) as i32 - age;
            } else {
                next_spoken = 0;
            }
            age_map.insert(spoken, (i + 1) as i32);
            println!("spoken:{:?} - {:?}", i + 1, spoken);
        }

        spoken
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::common;

    #[test]
    fn day15_part1() {
        // let list = common::parse_from_file("./data/day15_test.txt");
        // let list = common::parse_from_file("./data/day15_part1.txt");
        // let v = day15::part1(Box::new(list.unwrap()));
        // let v = day15::part1("0,3,6".to_string());
        // let v = day15::part1("3,1,2".to_string());
        // let v = day15::part1("2,1,3".to_string());
        let v = day15::part1("19,20,14,0,9,1".to_string(), 2020);
        // let v = day15::part1("2,1,3".to_string());
        assert_eq!(v, 1325);
    }

    #[test]
    fn day15_part2() {
        // let list = common::parse_from_file("./data/day15_test.txt");
        // let list = common::parse_from_file("./data/day15_part1.txt");
        // let v = day15::part1(Box::new(list.unwrap()));
        // let v = day15::part1("0,3,6".to_string(), 2020);
        // let v = day15::part1("0,3,6".to_string(), 30000000);
        // let v = day15::part1("3,1,2".to_string());
        // let v = day15::part1("2,1,3".to_string());
        let v = day15::part1("19,20,14,0,9,1".to_string(), 30000000);
        // let v = day15::part1("2,1,3".to_string());
        assert_eq!(v, 1325);
    }
}
