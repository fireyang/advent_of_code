mod day10 {
    use std::collections::HashMap;


    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let mut v:Vec<i32> =vec.iter().map(|it|it.parse().unwrap()).collect();
        v.sort();
        v.push(v[v.len()-1] +3);
        let mut map = HashMap::<i32, i32>::new();
        let mut jolt = 0;
        for it in v.iter() {
            // println!("{:?}", it);
            let offset  = it - jolt;
            let v = map.entry(offset).or_insert(0);
            *v +=1;
            jolt = *it
        }
        println!("map, {:?}", map);
        map.get(&1).unwrap() * map.get(&3).unwrap()
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> i64 {
        let mut v:Vec<i32> =vec.iter().map(|it|it.parse().unwrap()).collect();
        v.sort();
        v.insert(0, 0);
        v.push(v[v.len()-1] +3);
        let mut res = vec![];
        let mut jolt = 0;
        let mut sub_v = vec![];

        for it in v.iter() {
            let offset  = it - jolt;
            if offset == 3{
                if sub_v.len() > 2{
                    res.push(sub_v.clone());
                }
                sub_v = vec![*it];
                // start = it;
            }else{
                sub_v.push(*it);
            }
            jolt = *it;
        }
        let mut all:i64 = 1;
        for i in res{
            let r = match i.len()-2 {
                3 => 7,
                2 => 4,
                1 => 2,
                _ => 1,
            };
            all *= r;
        }
        all
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day10_part1() {
        // let list = common::parse_from_file("./data/day10_test.txt");
        let list = common::parse_from_file("./data/day10_part1.txt");
        let v = day10::part1(Box::new(list.unwrap()));
        // let list = common::parse_from_file("./data/day10_part1.txt");
        // let v = day10::part1(Box::new(list.unwrap()), 25);
        assert_eq!(v, 2484);
    }

    #[test]
    fn day10_part2() {
        // let list = common::parse_from_file("./data/day10_test.txt");
        let list = common::parse_from_file("./data/day10_part1.txt");
        let v = day10::part2(Box::new(list.unwrap()));
        // let list = common::parse_from_file("./data/day10_part1.txt");
        // let v = day10::part1(Box::new(list.unwrap()), 25);
        assert_eq!(v, 15790581481472);
    }

}
