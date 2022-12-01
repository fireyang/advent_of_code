mod day1 {
    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                if v[i] + v[j] == 2020 {
                    println!("out:{:?},{:?}", i, j);
                    return v[i] * v[j];
                }
            }
        }
        0
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        // println!("out:{:?}", v);
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                for k in j + 1..v.len() {
                    if v[i] + v[j] + v[k] == 2020 {
                        println!("out:{:?}", (i, j, k));
                        return v[i] * v[j] * v[k];
                    }
                }
            }
        }
        0
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
        assert_eq!(fuels, 32064);
    }

    #[test]
    fn day1_part2() {
        let list = common::parse_from_file("./data/day1_part1.txt");
        let fuels = day1::part2(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(fuels, 193598720);
    }
}
