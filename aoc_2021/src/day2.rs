mod day2 {
    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let v: Vec<(&str, i32)> = vec
            .iter()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
            // .map(|x| (x.0, x.1.parse::<i32>().unwrap()))
            .collect();
        let mut x = 0;
        let mut y = 0;
        for (_, (a, b)) in v.iter().enumerate() {
            match *a {
                "forward" => x += b,
                "down" => y += b,
                "up" => y -= b,
                _ => y = y,
            }
        }
        // println!("{:?}", (x, y));
        x * y
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let v: Vec<(&str, i32)> = vec
            .iter()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
            // .map(|x| (x.0, x.1.parse::<i32>().unwrap()))
            .collect();
        // println!("{:?}", v);
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;
        for (_, (a, b)) in v.iter().enumerate() {
            match *a {
                "forward" => {x += b;y +=aim*b},
                "down" => aim += b,
                "up" => aim -= b,
                _ => y = y,
            }
        }
        // println!("{:?}", (x, y));
        x * y
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day1_part1() {
        // let list = common::parse_from_file("./data/day2_test.txt");
        let list = common::parse_from_file("./data/day2_part1.txt");
        let ret = day2::part1(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(ret, 2019945);
    }

    #[test]
    fn day1_part2() {
        // let list = common::parse_from_file("./data/day2_test.txt");
        let list = common::parse_from_file("./data/day2_part1.txt");
        let ret = day2::part2(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(ret, 1599311480);
    }
}
