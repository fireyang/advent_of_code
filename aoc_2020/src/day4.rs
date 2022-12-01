extern crate regex;
mod day4 {

    use regex::Regex;
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let mut count = 0;
        vec.split(|line| line.len() == 0)
            .map(|x| {
                x.iter()
                    .map(|sub| {
                        sub.split(|x| x == ' ' || x == ':')
                            .collect::<Vec<_>>()
                            .chunks(2)
                            .map(|it| (it[0], it[1]))
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<HashMap<_, _>>()
            })
            .for_each(|x| {
                if x.len() == 8 || (x.len() == 7 && x.get("cid") == None) {
                    count += 1;
                }
            });
        count
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> i32 {
        let color_re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        let pid_re = Regex::new(r"^[a-f0-9]{9}$").unwrap();
        let elc_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let mut count = 0;
        vec.split(|line| line.len() == 0)
            .map(|x| {
                x.iter()
                    .map(|sub| {
                        sub.split(|x| x == ' ' || x == ':')
                            .collect::<Vec<_>>()
                            .chunks(2)
                            .map(|it| (it[0], it[1]))
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<HashMap<_, _>>()
            })
            .for_each(|x| {
                if x.len() == 8 || (x.len() == 7 && x.get("cid") == None) {
                    let ok: bool = x.iter().all(|it| match it {
                        (&"byr", v) => {
                            let r = v.parse::<i32>().unwrap();
                            r >= 1920 && r <= 2002
                        }
                        (&"iyr", v) => {
                            let r = v.parse::<i32>().unwrap();
                            r >= 2010 && r <= 2020
                        }
                        (&"eyr", v) => {
                            let r = v.parse::<i32>().unwrap();
                            r >= 2020 && r <= 2030
                        }
                        (&"hgt", v) => match v.split_at(v.len() - 2) {
                            (v, "cm") => {
                                let r = v.parse::<i32>().unwrap();
                                r >= 150 && r <= 193
                            }
                            (v, "in") => {
                                let r = v.parse::<i32>().unwrap();
                                r >= 59 && r <= 76
                            }
                            _ => false,
                        },
                        (&"hcl", v) => color_re.is_match(v),
                        (&"ecl", v) => elc_colors.contains(v),
                        (&"pid", v) => pid_re.is_match(v),
                        (&"cid", _) => true,
                        _ => false,
                    });
                    if ok {
                        count += 1;
                    } else {
                        // println!("{:?}", x);
                    }
                }
            });
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day4_part1() {
        // let list = common::parse_from_file("./data/day4_test.txt");
        // let result = day4::part1(Box::new(list.unwrap()));
        // assert_eq!(result, 2);
        let list = common::parse_from_file("./data/day4_part1.txt");
        let result = day4::part1(Box::new(list.unwrap()));
        assert_eq!(result, 235);
    }

    #[test]
    fn day4_part2() {
        let list = common::parse_from_file("./data/day4_part1.txt");
        let result = day4::part2(Box::new(list.unwrap()));
        assert_eq!(result, 194);
    }
}
