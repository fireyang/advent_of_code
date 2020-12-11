mod day11 {
    use std::collections::HashMap;
    use std::rc::Rc;

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        println!("hello");
        let mut map: HashMap<(usize, usize), char> = HashMap::new();
        let mut width = 0;
        vec.iter().enumerate().for_each(|(y, it)| {
            width = it.len();
            it.chars().enumerate().for_each(|(x, it2)| {
                let mut c = it2;
                if c == 'L' {
                    c = '#';
                }
                // println!("it2:{:?}", (x, y, it2));
                map.insert((x, y), c);
            })
        });
        // println!("map, {:?}", map);
        let mut change = true;
        let mut change_num = 0;
        let mut map = Rc::new(map);
        while change {
            // _print_map(map.clone(), width, "aaa");
            let (c, new_map) = change_pos(map.clone());
            // println!("cc, {:?}", (change_num, c));
            change = c;
            map = Rc::new(new_map);
            change_num += 1;
            // _print_map(map.clone(), width, "bbb");
        }
        let c = map.iter().map(|(_, v)| v).filter(|&x| x == &'#').count();
        println!("map2, {:?}", (change_num, c));
        0
    }

    fn _print_map(map: Rc<HashMap<(usize, usize), char>>, w: usize, key: &str) {
        println!("==========={:?}===========", key);
        for i in 0..w {
            let mut v = vec![];
            for j in 0..map.len() / w {
                v.push(map[&(j, i)]);
            }
            let s: String = v.into_iter().collect();
            println!("{:?}", s);
        }
        println!("======================");
    }

    pub fn change_pos(
        map: Rc<HashMap<(usize, usize), char>>,
    ) -> (bool, HashMap<(usize, usize), char>) {
        let mut new_map: HashMap<(usize, usize), char> = HashMap::new();
        let mut all_change = false;
        for ((x, y), c1) in map.iter() {
            if c1 == &'.' {
                new_map.insert((*x, *y), *c1);
                continue;
            }
            let mut adjacent = vec![];
            for i in 0..3 {
                for j in 0..3 {
                    if i == j && i == 1 {
                        continue;
                    }
                    if x + i == 0 || y + j == 0 {
                        continue;
                    }
                    let px = x + i - 1;
                    let py = y + j - 1;
                    if let Some(c2) = map.get(&(px, py)) {
                        if c2 != &'.' {
                            adjacent.push(*c2);
                        }
                    }
                }
            }
            let change: bool;
            if c1 == &'L' {
                change = adjacent.iter().all(|x| x == &'L');
            } else {
                change = adjacent.iter().filter(|&x| x == &'#').count() > 3;
            }

            let mut new_c = *c1;
            if change {
                if new_c == '#' {
                    new_c = 'L';
                } else {
                    new_c = '#';
                }
                all_change = true;
            }
            new_map.insert((*x, *y), new_c);
        }
        (all_change, new_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day11_part1() {
        // let list = common::parse_from_file("./data/day11_test.txt");
        let list = common::parse_from_file("./data/day11_part1.txt");
        let v = day11::part1(Box::new(list.unwrap()));
        // let list = common::parse_from_file("./data/day11_part1.txt");
        // let v = day11::part1(Box::new(list.unwrap()), 25);
        assert_eq!(v, 2484);
    }
}
