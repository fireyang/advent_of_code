#[allow(dead_code)]
mod day1 {

    #[derive(Debug)]
    struct Elf {
        calories: Vec<i32>,
    }

    impl Elf {
        fn sum(&self) -> i32 {
            let mut total = 0;
            for x in &self.calories {
                total += x;
            }
            total
        }
    }

    fn gen_elf(vec: Vec<String>) -> Vec<Elf> {
        let mut elf_list = Vec::new();
        let mut calories = vec![];
        for x in &vec {
            match x.as_str() {
                "" => {
                    elf_list.push(Elf { calories: calories });
                    calories = vec![];
                }
                _ => {
                    let v = x.parse().unwrap();
                    calories.push(v);
                }
            }
        }
        if calories.len() > 0 {
            elf_list.push(Elf { calories: calories });
        }
        elf_list
    }

    pub fn part1(vec: Vec<String>) -> i32 {
        let elf_list = gen_elf(vec);
        let mut most_calories = 0;
        for it in &elf_list {
            // println!("{:?}", it.sum());
            let v = it.sum();
            if most_calories < v {
                most_calories = v;
            }
        }
        // println!("{:?}", elf_list);
        most_calories
    }

    pub fn part2(vec: Vec<String>) -> i32 {
        let elf_list = gen_elf(vec);
        let mut top_vec: Vec<i32> = elf_list.iter().map(|x| x.sum()).collect();
        // println!("{:?}", top_vec);
        top_vec.sort_by(|a, b| b.cmp(a));
        // println!("{:?}", top_vec.get(..3));
        let most_calories = top_vec.get(..3).unwrap().iter().sum();
        // println!("{:?}", most_calories);
        most_calories
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate common;

    #[test]
    fn day1_part1() {
        // let list = common::parse_from_file("./data/day1_part1.txt");
        let list = common::parse_from_file("./data/day1_test.txt");
        let mut ret = day1::part1(list.unwrap());
        assert_eq!(ret, 24000);
        let list2 = common::parse_from_file("./data/day1_part1.txt");
        ret = day1::part1(list2.unwrap());
        assert_eq!(ret, 71506);
    }
    #[test]
    fn day1_part2() {
        // let list = common::parse_from_file("./data/day1_part1.txt");
        let list = common::parse_from_file("./data/day1_test.txt");
        let mut ret = day1::part2(list.unwrap());
        assert_eq!(ret, 45000);
        let list2 = common::parse_from_file("./data/day1_part1.txt");
        ret = day1::part2(list2.unwrap());
        assert_eq!(ret, 209603);
    }
}
