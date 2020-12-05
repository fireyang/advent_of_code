// extern crate regex;
mod day5 {

    // use std::collections::HashMap;
    // use regex::Regex;

    #[allow(dead_code)]
    pub fn part1(seat_str: String) -> i32 {
        let c_list:Vec<char> = seat_str.chars().collect();
        let (rows, cols) = c_list.split_at(7);
        let mut start = 0;
        let mut end = 127;
        for it in rows{
            // let ok = (start+end)%2;
            let mid = (start+end)/2;
            match it {
                'F' => end = mid ,
                'B' => start = mid +1,
                _ => (),
            }
            // println!("{:?}", (start, end, it));
        }
        let row = if start > end{ end }else {start};
        start = 0;
        end = 7;
        for it in cols{
            let mid = (start+end)/2;
            match it {
                'L' => end = mid ,
                'R' => start = mid +1,
                _ => (),
            }
            // println!("{:?}", (start, end, it));
        }
        let col = if start > end{ end }else {start};
        let v = row * 8 + col;
        // println!("{:?}", v);
        v
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day5_part1() {
        // let v = "FBFBBFFRLR";
        // let result = day5::part1(v.to_string());
        // day5::part1("BFFFBBFRRR".to_string());
        // day5::part1("FFFBBBFRRR".to_string());
        // day5::part1("BBFFBBFRLL".to_string());
        let list = common::parse_from_file("./data/day5_part1.txt");
        let mut max = 0;
        for it in list.unwrap(){
            let v = day5::part1(it.clone());
            // println!("{:?}", (&it, v));
            if max < v{
                max = v;
            }
        }

        assert_eq!(max, 871);
    }

    #[test]
    fn day5_part2() {
        let list = common::parse_from_file("./data/day5_part1.txt");
        let mut vec = Vec::new();
        for it in list.unwrap(){
            let v = day5::part1(it.clone());
            vec.push(v);
        }
        vec.sort();
        let mut cur = 0;
        for val in vec.iter(){
            if cur !=0 && cur +1 != *val{
                break;
            }else{
                cur = *val;
            }
        }
        cur += 1;
        println!("out:{:?}", cur);

        assert_eq!(cur, 640);
    }

}
