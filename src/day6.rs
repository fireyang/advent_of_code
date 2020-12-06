// extern crate regex;
mod day6 {

    use std::collections::HashMap;
    use std::collections::HashSet;
    // use regex::Regex;

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let v :Vec<_>=
        vec.split(|line| line.len()  == 0)
        .map(|x|{
            // println!("x:{:?}", x);
            let mut set :HashSet<char> = HashSet::new();
            x.iter().for_each(|s|{
                // println!("s:{:?}", s);
                s.chars().for_each(|c|{set.insert(c);});
            });
            set.len() as i32
        })
        .collect();
        v.iter().sum()
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> i32 {
        let v :Vec<_>=
        vec.split(|line| line.len()  == 0)
        .map(|x|{
            let s = x.len() as i32;
            let mut map :HashMap<char, i32> = HashMap::new();
            x.concat().chars().for_each(|c|{
                let count = map.entry(c).or_insert(0);
                *count += 1;
            });
            let mut c = 0;
            map.iter().for_each(|(_, v)|
                if *v == s{
                    c +=1;
                });
            c

        })
        .collect();
        v.iter().sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day6_part1() {
        // let list = common::parse_from_file("./data/day6_test.txt");
        let list = common::parse_from_file("./data/day6_part1.txt");
        let result = day6::part1(Box::new(list.unwrap()));

        assert_eq!(result, 6259);
    }

    #[test]
    fn day6_part2() {
        // let list = common::parse_from_file("./data/day6_test.txt");
        let list = common::parse_from_file("./data/day6_part1.txt");
        let result = day6::part2(Box::new(list.unwrap()));

        assert_eq!(result, 3178);
    }

}
