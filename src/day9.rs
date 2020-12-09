mod day9 {


    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>, preamble_len: usize) -> i64 {
        let v:Vec<i64> =vec.iter().map(|it|it.parse().unwrap()).collect();
        for check in 0..vec.len()-preamble_len{
            let mut is_valid = false;
            let check_pos = check + preamble_len;
            for i in check..check_pos{
                for j in i+1..check_pos{
                    if v[i] + v[j] == v[check_pos]{
                        // println!("{:?}", (check_pos,v[check_pos], v[i] , v[j] ));
                        is_valid = true;
                        break;
                    }
                }
                if is_valid {
                    break;
                }
            }
            if !is_valid{
                println!("{:?}", (check_pos, v[check_pos], is_valid));
                return v[check_pos];
            }
        }
        0
    }


    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>, pos: usize) -> i64 {
        let v:Vec<i64> =vec.iter().map(|it|it.parse().unwrap()).collect();
        let mut start = 0;
        let mut end = start+1;
        let mut all = v[start]+v[end];
        let check_val = v[pos];
        while all != check_val || end >= vec.len(){
            if all < check_val{
                end +=1;
                all += v[end];
            }else if all > check_val{
                all -= v[start];
                start +=1;
            }
            // println!("{:?}", (start, end, all));
        }
        println!("{:?}", (start, end, v[start], v[end], all, check_val));
        let mut min = 0;
        let mut max = 0;
        for i in start..end{
            if v[i] > max{
                max =v[i]
            }
            if min==0 || min > v[i]{
                min = v[i]
            }
        }
        min+max
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day9_part1() {
        // let list = common::parse_from_file("./data/day9_test.txt");
        // let v = day9::part1(Box::new(list.unwrap()), 5);
        let list = common::parse_from_file("./data/day9_part1.txt");
        let v = day9::part1(Box::new(list.unwrap()), 25);
        assert_eq!(v, 26796446);
    }

    #[test]
    fn day9_part2() {
        let list = common::parse_from_file("./data/day9_part1.txt");
        // let list = common::parse_from_file("./data/day9_test.txt");
        let v = day9::part2(Box::new(list.unwrap()), 508);

        assert_eq!(v, 3353494);
    }

}
