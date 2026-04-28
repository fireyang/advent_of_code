mod day3 {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        // let mut ret: Vec
        let v: Vec<Vec<u32>> = vec
            .iter()
            .map(|x| x.chars().map(|v|v.to_digit(10).unwrap()).collect())
            .collect();
        let mut ret: Vec<u32> = vec!(0; v[0].len().try_into().unwrap());
        for it in v.iter() {
            // println!("{:?}", it);
            // println!("{:?}", ret);
            it.iter().enumerate().for_each(|(i, x)| ret[i] += x);
        }
        let mut gamma_rate  = 0;
        let mut epsilon_rate  = 0;
        // println!("{:?}", v);
        let kk:u32 = (v.len()/2).try_into().unwrap();
        for it in ret.iter() {
            // println!("{:?}, {:?}", it, kk);
            gamma_rate <<= 1;
            epsilon_rate  <<= 1;
            if it > &kk {
                gamma_rate+=1
            }else{
                epsilon_rate+=1
            }
            // println!("{:?}", ret_num);
        }
        // println!("{:?}", v);
        println!("{:?}, {:?}", gamma_rate, epsilon_rate);
        gamma_rate * epsilon_rate
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        // let mut ret: Vec
        let v: Vec<Vec<u32>> = vec
            .iter()
            .map(|x| x.chars().map(|v|v.to_digit(10).unwrap()).collect())
            .collect();
        let cur:Vec<Vec<u32>> = v;
        let mut step = 0;
        let mut is_finish = false;
        while !is_finish {
            let mut sum = 0;
            for it in cur.iter() {
                if it[step] > 0 {
                    sum += 1;
                }
            }
            if sum > cur.len()/2 {
                cur = cur.filter(|x| x[step] > 0).collect();
            }
            is_finish = true;
        }
        // for it in v.iter() {
        // let kk:u32 = (v.len()/2).try_into().unwrap();
        //
        // }
        0
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day3_part1() {
        // let list = common::parse_from_file("./data/day3_test.txt");
        let list = common::parse_from_file("./data/day3_part1.txt");
        let ret = day3::part1(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(ret, 3277364);
    }

    #[test]
    fn day3_part2() {
        let list = common::parse_from_file("./data/day3_test.txt");
        // let list = common::parse_from_file("./data/day3_part1.txt");
        let ret = day3::part2(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(ret, 3277364);
    }

}
