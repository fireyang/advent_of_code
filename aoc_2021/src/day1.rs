
mod day1 {
    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        // println!("{:?}", v);
        let mut prev = -1;
        let mut inc_count = 0;
        for i in 0..v.len() {
            if prev != -1 {
                if v[i] > prev {
                    inc_count += 1;
                }
            }
            prev = v[i];
        }
        println!("{}", inc_count);
        inc_count
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        // println!("{:?}", v);
        let mut prev = -1;
        // let mut wind = vec![0; 3];
        let mut wind_sum ;
        let mut inc_count = 0;
        let len = v.len() - 2;
        for i in 0..len{
            wind_sum = v[i] + v[i + 1] + v[i + 2] ;
            if prev != -1 {
                if wind_sum > prev {
                    inc_count += 1;
                }
            }
            // println!("{}", wind_sum);
            prev = wind_sum;
        }
        // println!("{}", inc_count);
        inc_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day1_part1() {
        let list = common::parse_from_file("./data/day1_part1.txt");
        let fuels = day1::part1(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(fuels, 1553);
    }

    #[test]
    fn day1_part2() {
        // let list = common::parse_from_file("./data/day1_test.txt");
        let list = common::parse_from_file("./data/day1_part1.txt");
        let fuels = day1::part2(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(fuels, 1597);
    }
}
