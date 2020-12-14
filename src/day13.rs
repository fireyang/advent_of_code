mod day13 {

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let air_st:i32 = vec[0].parse().unwrap();
        let list:Vec<(i32, i32)> = vec[1].split(&",")
            .filter(|s| s != &"x")
            .map(|s|{
                let v:i32 = s.parse().unwrap();
                let low = air_st / v;
                let offset = (low +1) * v - air_st;
                // println!("{:?}", (offset, v));
                (offset, v)
            }).collect();
        let mut min = 0;
        let mut b = 0;
        for (v, base) in list.iter(){
            if min ==0 || min > *v{
                min = *v;
                b = *base
            }
        }
        println!("sdf:{:?}", (b, min));
        b * min
        // find.unwrap()
    }

    #[allow(dead_code)]
    pub fn part2(in_str: String) -> i32 {
        let list:Vec<(i64, i64)> = in_str.split(&",")
            .enumerate()
            .filter(|(_, s)| s != &"x")
            .map(|(idx, s)|{
                let v:i64 = s.parse().unwrap();
                (idx as i64, v)
            }).collect();
        // let step:i32 = list.iter().map(|(_, v)|*v).max().unwrap();
        let step:(i64, i64) = list.iter()
        .fold((0, 0), |acc, x| {
            if acc.1 > x.1{
                acc
            }else{
                *x
            }
        });
        // let step:i64 = v.1;
        // let step:i32 = list[0].1;
        let mut next:i64 = 0;
        let mut find = false;
        let mut idx =  100000000000000 / step.1;
        // let mut idx =  0;
        println!("v:{:?}", list);
        println!("v2:{:?}", (step, idx));
        while !find{
            idx += 1;
            next = idx * step.1 - step.0;
            find = list.iter().all(|(idx, v)|{
                let ret = ((next + *idx) % *v) == 0;
                // println!("v:{:?}", (next, idx, v, ret ));
                ret
            });
            if idx % 1000000 == 0{
                println!("v2:{:?}", (idx, next, find));
            }
            // if idx >1 {
                // find = true
            // }
        }
        println!("out:{:?}", (idx, next ));
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    // #[test]
    // fn day13_part1() {
    //     // let list = common::parse_from_file("./data/day13_test.txt");
    //     let list = common::parse_from_file("./data/day13_part1.txt");
    //     let v = day13::part1(Box::new(list.unwrap()));
    //     assert_eq!(v, 174);
    // }

    #[test]
    fn day13_part2() {
        // let list = common::parse_from_file("./data/day13_test.txt");
        let list = common::parse_from_file("./data/day13_part1.txt");
        // let v = day13::part2("67,7,59,61".to_string());
        // day13::part2("7,13,x,x,59,x,31,19".to_string());
        // day13::part2("17,13".to_string());
        // let v = day13::part2("7,x,x,13".to_string());
        // day13::part2("67,7,59,61".to_string());
        // day13::part2("67,x,7,59,61".to_string());
        let v = day13::part2(list.unwrap()[1].clone());
        assert_eq!(v, 998);
    }

}
