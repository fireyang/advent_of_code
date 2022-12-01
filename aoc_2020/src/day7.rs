// extern crate regex;
mod day7 {
    use std::cell::RefCell;
    use std::rc::Rc;

    use std::collections::HashMap;
    use std::collections::HashSet;

    #[derive(Clone, Debug)]
    struct Bag {
        num: i32,
        color: String,
    }

    fn _gen_deps(vec: Box<Vec<String>>) -> Vec<(String, Vec<(i32, String)>)> {
        let chan: Vec<_> = vec
            .iter()
            .map(|x| {
                let x = x.replace("bags", "").replace("bag", "").replace(".", "");
                let v1: Vec<&str> = x.split("contain").collect();
                let x1 = v1.get(0).unwrap().trim();
                let next = v1.get(1).unwrap();
                let bags: Vec<_> = next
                    .split(",")
                    .map(|v| {
                        let bags: Vec<&str> = v.trim().splitn(2, " ").collect();
                        let n = bags[0];
                        match n {
                            "no" => (0, "".to_string()),
                            _ => {
                                let num = bags.get(0).unwrap().parse::<i32>().unwrap();
                                let color = bags.get(1).unwrap().trim();
                                (num, color.to_string())
                                // let list =  map.entry(color).or_insert(Vec::new()).push("test");
                                // list.push(x1.to_string());
                            }
                        }
                    })
                    .collect();
                (x1.to_string(), bags)
            })
            .collect();
        chan
    }

    fn _belong_map(vec: Box<Vec<String>>) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let chan: Vec<_> = _gen_deps(vec);
        for it in chan.iter() {
            let (color1, bags) = it;
            for it2 in bags.iter() {
                match it2 {
                    (0, _) => (),
                    (_num, color) => {
                        let list = map.entry(color.to_string()).or_insert(Vec::new());
                        list.push(color1.to_string());
                    }
                }
            }
        }
        map
    }

    fn _include_map(vec: Box<Vec<String>>) -> HashMap<String, Vec<Bag>> {
        let mut map: HashMap<String, Vec<Bag>> = HashMap::new();
        let chan: Vec<_> = _gen_deps(vec);
        for it in chan.iter() {
            let (color1, bags) = it;
            for it2 in bags.iter() {
                match it2 {
                    (0, _) => (),
                    (num, color) => {
                        let list = map.entry(color1.to_string()).or_insert(Vec::new());
                        let bag = Bag {
                            num: *num,
                            color: color.to_string(),
                        };
                        list.push(bag);
                    }
                }
            }
        }
        map
    }

    fn get_parent_num(
        map: Box<HashMap<String, Vec<String>>>,
        key: &String,
        out: Rc<RefCell<HashSet<String>>>,
    ) {
        if let Some(bags) = map.get(key) {
            bags.iter().for_each(|it| {
                out.borrow_mut().insert(it.to_string());
                get_parent_num(map.clone(), &it, out.clone());
            });
        }
    }

    fn _get_child_num(map: Rc<HashMap<String, Vec<Bag>>>, key: &String) -> i32 {
        let mut count = 0;
        if let Some(bags) = map.get(key) {
            bags.iter().for_each(|it| {
                let v = _get_child_num(map.clone(), &it.color);
                count += v * it.num + it.num;
            });
        }
        count
    }

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let set: Rc<RefCell<_>> = Rc::new(RefCell::new(HashSet::new()));
        let map = _belong_map(vec);
        get_parent_num(Box::new(map), &"shiny gold".to_string(), set.clone());
        let v = set.borrow().len() as i32;
        v
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>, bag: &String) -> i32 {
        let map = _include_map(vec);
        let v = _get_child_num(Rc::new(map), bag);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day7_part1() {
        // let list = common::parse_from_file("./data/day7_test.txt");
        let list = common::parse_from_file("./data/day7_part1.txt");
        let v = day7::part1(Box::new(list.unwrap()));

        assert_eq!(v, 337);
    }

    #[test]
    fn day7_part2() {
        let list = common::parse_from_file("./data/day7_part1.txt");
        // let list = common::parse_from_file("./data/day7_test.txt");
        let v = day7::part2(Box::new(list.unwrap()), &"shiny gold".to_string());

        assert_eq!(v, 50100);
    }
}
