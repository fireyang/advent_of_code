mod day16 {
    // use std::collections::HashMap;
    use std::collections::LinkedList;
    // use std::collections::HashSet;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    // #[derive(Debug)]
    // struct Node{
    //     value: i32,
    //     end: i32,
    //     pre: Option<Weak<RefCell<Node>>>,
    //     next: Option<Rc<RefCell<Node>>>,
    // }
    //
    // impl Node{
    //     fn new(s:i32, e:i32) -> RefCell<Node>{
    //         RefCell::new(Node{
    //             value: s,
    //             end: e,
    //             pre: None,
    //             next:None,
    //         })
    //     }
    // }
    //
    // #[derive(Debug)]
    // pub struct DList{
    //     count: i32,
    //     first: Option<Rc<RefCell<Node>>>,
    //     last: Option<Rc<RefCell<Node>>>
    // }
    //
    // impl DList{
    //     pub fn new() -> DList{
    //         DList {
    //             count: 0,
    //             first: None,
    //             last: None,
    //         }
    //     }
    //
    //     pub fn add(&mut self, s:i32, e:i32) {
    //         let mut find = false;
    //         let cur = self.first;
    //         while !find{
    //             if Some(it) = cur{
    //                 if it.value < s{
    //                 }
    //             }else{
    //                 cur = Some(Rc::new<Node::new()>);
    //                 find = true;
    //             }
    //         }
    //     }
    // }

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let mut stage = 0;
        let mut list: Vec<(i32, i32)> = vec![];
        let mut nearby_tickets: Vec<i32> = vec![];
        for s in vec.iter() {
            if s == "your ticket:" {
                stage = 1;
            } else if s == "nearby tickets:" {
                stage = 2;
            } else if s.len() > 0 {
                match stage {
                    0 => {
                        let rules: Vec<String> = s
                            .replace(" or ", ":")
                            .split(':')
                            .map(|x| x.trim().to_string())
                            .collect();
                        println!("{:?}", rules);
                        // let s: i32= rules[1].parse().unwrap();
                        // let e: i32 = rules[2].parse().unwrap();
                        // let v:(String, i32, i32) = (rules[0].to_string(), s,e );
                        let (s, e) = get_range(&rules[1]);
                        list.push((s, 0));
                        list.push((e, 1));
                        let (s, e) = get_range(&rules[2]);
                        list.push((s, 0));
                        list.push((e, 1));
                    }
                    2 => {
                        s.split(',')
                            .map(|x| x.parse::<i32>().unwrap())
                            .for_each(|x| nearby_tickets.push(x));
                    }
                    _ => (),
                }
            }
        }

        list.sort_by(|&a, &b| a.0.partial_cmp(&b.0).unwrap());

        let mut range_list: Vec<(i32, i32)> = Vec::with_capacity(list.len() / 2);
        let mut cur = (0, 0);
        for it in list.iter() {
            // println!("{:?}",(cur, it));
            if cur.1 != 0 && it.1 == 0 {
                range_list.push(cur);
                cur = (0, 0);
            }
            if cur.0 == 0 && it.1 == 0 {
                cur.0 = it.0;
            } else if it.1 == 1 {
                cur.1 = it.0;
            }
        }
        if cur.1 != 0 {
            range_list.push(cur);
        }
        // println!("{:?}", list);
        let left: i32 = nearby_tickets
            .iter()
            .filter(|x| !range_list.iter().any(|it| *x >= &it.0 && *x <= &it.1))
            .sum();
        println!("{:?}", range_list);
        println!("{:?}", left);
        0
    }

    fn get_range(s: &String) -> (i32, i32) {
        let rules: Vec<i32> = s.split('-').map(|x| x.trim().parse().unwrap()).collect();
        (rules[0], rules[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day16_part1() {
        // let list = common::parse_from_file("./data/day16_test.txt");
        let list = common::parse_from_file("./data/day16_part1.txt");
        // let v = day16::part1(Box::new(list.unwrap()));
        // let v = day16::part1("0,3,6".to_string());
        // let v = day16::part1("3,1,2".to_string());
        // let v = day16::part1("2,1,3".to_string());
        let v = day16::part1(list.unwrap());
        // let v = day16::part1("2,1,3".to_string());
        assert_eq!(v, 1325);
    }
}
