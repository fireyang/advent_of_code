mod day16 {
    use std::collections::HashMap;
    use std::collections::HashSet;

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
                        // println!("{:?}", rules);
                        // let s: i32= rules[1].parse().unwrap();
                        // let e: i32 = rules[2].parse().unwrap();
                        // let v:(String, i32, i32) = (rules[0].to_string(), s,e );
                        list.push(get_range(&rules[1]));
                        list.push(get_range(&rules[2]));
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

        let mut range_list: Vec<(i32, i32)> = Vec::with_capacity(list.len());
        // let mut tail:Option<(i32, i32)> = None;
        for it in list.iter() {
            let v = range_list.len();
            if v > 0 {
                let it2 = range_list[v - 1];
                if it2.1 < it.0 {
                    range_list.push(*it);
                } else {
                    range_list[v - 1] = (it2.0, it.1);
                    // *it2.1 = it.1;
                }
            } else {
                range_list.push(*it);
            }
        }
        // println!("{:?}", list);
        let left: i32 = nearby_tickets
            .iter()
            .filter(|x| !range_list.iter().any(|it| *x >= &it.0 && *x <= &it.1))
            .sum();
        // println!("{:?}",  range_list);
        // println!("{:?}",  left);
        left
    }

    type Range = (i32, i32);

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let mut stage = 0;
        let mut list: Vec<(String, Range, Range)> = vec![];
        let mut tickets: Vec<Vec<i32>> = vec![];
        let mut field_map: HashMap<String, HashSet<i32>> = HashMap::new();
        let mut idx_map: HashMap<usize, usize> = HashMap::new();
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
                        field_map.insert(rules[0].clone(), HashSet::new());
                        list.push((
                            rules[0].clone(),
                            get_range2(&rules[1]),
                            get_range2(&rules[2]),
                        ));
                    }
                    _ => {
                        let v = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                        tickets.push(v);
                    } // _ =>()
                }
            }
        }
        let init_v = (1 << list.len()) - 1;

        // tickets.iter().for_each(|ticket| {
        //     ticket.iter().enumerate().for_each(|(idx, x)| {
        //         list.iter().for_each(|(key, r1, r2)| {
        //             let v = (x >= &r1.0 && x <= &r1.1) || (x >= &r2.0 && x <= &r2.1);
        //             if v {
        //                 field_map.get_mut(key).unwrap().insert(idx as i32);
        //             }
        //         });
        //     })
        // });

        tickets.iter().for_each(|ticket| {
            ticket.iter().enumerate().for_each(|(idx, x)| {
                for it in list.iter() {
                    let (_key, r1, r2) = it;
                    let v = (x >= &r1.0 && x <= &r1.1) || (x >= &r2.0 && x <= &r2.1);
                    if !v {
                        let v = idx_map.entry(idx).or_insert(init_v);
                        *v &= !(1 << idx);
                        // println!("222:{:?}, {:#b}, {:#b}, {:#b}", idx, v,init_v, !(1 << idx));
                    }
                }
            })
        });
        // for (k, v) in idx_map.iter(){
        //     println!("{:?}, {:#b} ", k,v);
        // }
        // for (idx, it) in list.enumerate().iter(){
        //     for (k, v) in idx_map.iter(){
        //         println!("{:?}, {:#b} ", k,v);
        //     }
        //     println!("{:?}, {:#b} ", k,v);
        // }
        // for it in list.iter(){
        //     for (idx, set) in idx_map.iter(){
        //         if !set.contains(&it.0){
        //             println!("{:?}", (idx, it.0.to_string()))
        //         }
        //     }
        // }

        // let valid_list :Vec<_>=  tickets.iter().filter(|ticket|{
        //     ticket.iter().all(|x|{
        //         list.iter().any(|(key, r1, r2)|{
        //             let v = (x >= &r1.0 && x <= &r1.1 )|| (x >= &r2.0 && x <= &r2.1);
        //             // println!("{:?}", (key, r1,r2, x, v));
        //             v
        //         })
        //     })
        // }).collect();
        // println!("{:?}", tickets);
        // println!("{:?}", field_map);
        // println!("{:?}",  valid_list);
        0
    }

    fn get_range2(s: &String) -> Range {
        let rules: Vec<i32> = s.split('-').map(|x| x.trim().parse().unwrap()).collect();
        (rules[0], rules[1])
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

    // #[test]
    // fn day16_part1() {
    //     // let list = common::parse_from_file("./data/day16_test.txt");
    //     let list = common::parse_from_file("./data/day16_part1.txt");
    //     // let v = day16::part1(Box::new(list.unwrap()));
    //     // let v = day16::part1("0,3,6".to_string());
    //     // let v = day16::part1("3,1,2".to_string());
    //     // let v = day16::part1("2,1,3".to_string());
    //     let v = day16::part1(list.unwrap());
    //     // let v = day16::part1("2,1,3".to_string());
    //     assert_eq!(v, 25972);
    // }

    #[test]
    fn day16_part2() {
        let list = common::parse_from_file("./data/day16_test2.txt");
        // let list = common::parse_from_file("./data/day16_part1.txt");
        // let v = day16::part1(Box::new(list.unwrap()));
        // let v = day16::part1("0,3,6".to_string());
        // let v = day16::part1("3,1,2".to_string());
        // let v = day16::part1("2,1,3".to_string());
        let v = day16::part2(list.unwrap());
        // let v = day16::part1("2,1,3".to_string());
        assert_eq!(v, 25972);
    }
}
