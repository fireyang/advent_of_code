mod day2 {

    fn get_inputs(vec: &Vec<String>) -> Vec<(i32, i32, char, &str)> {
        vec.iter()
            .map(|value| {
                let v: Vec<_> = value.split(|c| c == '-' || c == ':' || c == ' ').collect();
                // println!("{:?}", v);
                let min: i32 = v[0].parse().unwrap();
                let max: i32 = v[1].parse().unwrap();
                let c = v[2].chars().next().unwrap();
                (min, max, c, v[4])
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let v = get_inputs(&vec);
        let mut count = 0;
        for it in v.iter() {
            let (min, max, c, s) = it;
            let v = s.chars().enumerate().filter(|(_, x)| x == c).count() as i32;
            if v >= *min && v <= *max {
                count += 1;
            }
        }
        count
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let v = get_inputs(&vec);
        let mut count = 0;
        for it in v.iter() {
            let (min, max, c, s) = it;
            let s_char = s.chars().nth((*min - 1) as usize).unwrap();
            let e_char = s.chars().nth((*max - 1) as usize).unwrap();
            if (&s_char == c || &e_char == c) && s_char != e_char {
                // println!("{:?}", (it, s_char, e_char));
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day2_part1() {
        let list = common::parse_from_file("./data/day2_part1.txt");
        let fuels = day2::part1(list.unwrap());
        assert_eq!(fuels, 556);
    }

    #[test]
    fn day2_part2() {
        let list = common::parse_from_file("./data/day2_part1.txt");
        let fuels = day2::part2(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(fuels, 605);
    }
}
