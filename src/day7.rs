// extern crate regex;
mod day7 {
    use std::cell::{RefCell};
    use std::rc::Rc;

    use std::collections::HashMap;
    use std::collections::HashSet;
    // use regex::Regex;

    fn gen_map(vec: Box<Vec<String>>) -> HashMap<String, Vec<String>>{
        let mut map:HashMap<String, Vec<String>>= HashMap::new();
        let chan: Vec<_> = vec.iter()
            .map(|x|{
                let x = x.replace("bags", "").replace("bag", "").replace(".", "");
                let v1 :Vec<&str> = x.split("contain").collect();
                let x1 = v1.get(0).unwrap().trim();
                let next = v1.get(1).unwrap();
                let bags: Vec<_> = next.split(",")
                    .map(|v|{
                        let bags:Vec<&str> = v.trim().splitn(2, " ").collect();
                        let n = bags[0];
                        match n {
                            "no" => (0, "".to_string()),
                            _ => {
                                let num = bags.get(0).unwrap().parse::<i32>().unwrap();
                                let color = bags.get(1).unwrap().trim();
                                (num, color.to_string())
                                // let list =  map.entry(color).or_insert(Vec::new()).push("test");
                                // list.push(x1.to_string());
                            },
                        }
                    })
                    .collect();
                (x1.to_string(), bags)
            }).collect();
        for it in chan.iter(){
            let (color1, bags) = it;
            // println!("{:?}", it);
            for it2 in bags.iter(){
                match it2 {
                    (0, _) =>(),
                    (_num, color) => {
                        let list  =  map.entry(color.to_string()).or_insert(Vec::new());
                        list.push(color1.to_string());
                    }
                }
            }
            // println!("{:?}", (num, color));
        }
        map
    }

    fn get_parent_num(map: Box<HashMap<String, Vec<String>>>, key: &String, out:Rc<RefCell<HashSet<String>>>) {
        if let Some(keys) = map.get(key){
            keys.iter().for_each(|it| get_parent_num(map.clone(), it, out.clone()));
            for it in keys.iter(){
                out.borrow_mut().insert(it.to_string());
            }
        }
    }

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let set: Rc<RefCell<_>> = Rc::new(RefCell::new(HashSet::new()));
        let map = gen_map(vec);
        get_parent_num(Box::new(map), &"shiny gold".to_string(), set.clone());
        let v = set.borrow().len() as i32;
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

        assert_eq!(v, 871);
    }

    // #[test]
    // fn day7_part2() {
    //     let list = common::parse_from_file("./data/day7_part1.txt");
    //     let mut vec = Vec::new();
    //     for it in list.unwrap(){
    //         let v = day7::part1(it.clone());
    //         vec.push(v);
    //     }
    //     vec.sort();
    //     let mut cur = 0;
    //     for val in vec.iter(){
    //         if cur !=0 && cur +1 != *val{
    //             break;
    //         }else{
    //             cur = *val;
    //         }
    //     }
    //     cur += 1;
    //     println!("out:{:?}", cur);
    //
    //     assert_eq!(cur, 640);
    // }

}
