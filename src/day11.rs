mod day11 {

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        // let mut map: HashMap<(usize, usize), char> = HashMap::new();
        // let mut grids = vec![];
        let grids: Vec<Vec<char>> = vec
            .iter()
            .map(|it| {
                let row = it.chars().collect();
                row
            })
            .collect();
        // println!("map, {:?}", map);
        let mut change = true;
        let mut change_num = 0;
        let mut map = grids;
        while change {
            let (c, new_map) = change_pos(&map);
            change = c;
            map = new_map;
            change_num += 1;
            // _print_map(&map, "bbb");
        }
        let c = map.iter().flatten().filter(|&x| x == &'#').count();
        println!("map2, {:?}", (change_num, c));
        c as i32
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> i32 {
        let grids: Vec<Vec<char>> = vec
            .iter()
            .map(|it| {
                let row = it.chars().collect();
                row
            })
            .collect();
        // println!("map, {:?}", map);
        let mut change = true;
        let mut change_num = 0;
        let mut map = grids;
        while change {
            let (c, new_map) = change_pos2(&map);
            change = c;
            map = new_map;
            change_num += 1;
            // _print_map(&map, "bbb");
            // if change_num > 5 {
            //     change = false;
            // }
        }
        let c = map.iter().flatten().filter(|&x| x == &'#').count();
        println!("map2, {:?}", (change_num, c));
        c as i32
    }

    fn _print_map(v: &Vec<Vec<char>>, key: &str) {
        println!("==========={:?}===========", key);
        for it in v.iter() {
            let s: String = it.into_iter().collect();
            println!("{:?}", s);
        }
        println!("======================");
    }

    fn get_adjacent(vec: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
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
                if let Some(r) = vec.get(py) {
                    if let Some(v) = r.get(px) {
                        adjacent.push(*v);
                    }
                }
            }
        }
        adjacent
    }

    pub fn change_pos(grids: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
        let mut new_grids = grids.clone();
        let mut all_change = false;
        for (y, row) in grids.iter().enumerate() {
            for x in 0..row.len() {
                let c1 = grids[y][x];
                if c1 == '.' {
                    continue;
                }
                let adjacent = get_adjacent(&grids, x, y);
                let change: bool;
                if c1 == 'L' {
                    change = adjacent.iter().filter(|&x| x != &'.').all(|x| x == &'L');
                } else {
                    change = adjacent
                        .iter()
                        .filter(|&x| x != &'.')
                        .filter(|&x| x == &'#')
                        .count()
                        > 3;
                }

                let mut new_c = c1;
                if change {
                    if new_c == '#' {
                        new_c = 'L';
                    } else {
                        new_c = '#';
                    }
                    all_change = true;
                }
                new_grids[y][x] = new_c;
            }
        }
        (all_change, new_grids)
    }

    fn get_adjacent2(vec: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
        let mut adjacent = vec![];
        for i in 0..3 {
            for j in 0..3 {
                if i == j && i == 1 {
                    continue;
                }
                let c = get_pos(vec, x as i32, y as i32, i as i32 - 1, j as i32 - 1);
                adjacent.push(c);
            }
        }
        adjacent
    }

    fn get_pos(vec: &Vec<Vec<char>>, x: i32, y: i32, ox: i32, oy: i32) -> char {
        let h = vec.len() as i32;
        let w = vec[0].len() as i32;
        let mut find = true;
        let mut findchar = '.';
        let mut px = x;
        let mut py = y;
        while find {
            px += ox;
            py += oy;
            if px >= 0 && px < w && py >= 0 && py < h {
                findchar = vec[py as usize][px as usize];
                if findchar != '.' {
                    find = false;
                }
            } else {
                find = false;
            }
        }
        findchar
    }

    pub fn change_pos2(grids: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
        let mut new_grids = grids.clone();
        let mut all_change = false;
        for (y, row) in grids.iter().enumerate() {
            for x in 0..row.len() {
                let c1 = grids[y][x];
                if c1 == '.' {
                    continue;
                }
                let adjacent = get_adjacent2(&grids, x, y);
                let change: bool;
                if c1 == 'L' {
                    change = adjacent.iter().all(|x| x == &'L' || x == &'.');
                } else {
                    change = adjacent
                        .iter()
                        .filter(|&x| x != &'.')
                        .filter(|&x| x != &'L')
                        .count()
                        > 4;
                }

                let mut new_c = c1;
                if change {
                    if new_c == '#' {
                        new_c = 'L';
                    } else {
                        new_c = '#';
                    }
                    all_change = true;
                }
                new_grids[y][x] = new_c;
            }
        }
        (all_change, new_grids)
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
        assert_eq!(v, 2108);
    }

    #[test]
    fn day11_part2() {
        // let list = common::parse_from_file("./data/day11_test.txt");
        let list = common::parse_from_file("./data/day11_part1.txt");
        let v = day11::part2(Box::new(list.unwrap()));
        assert_eq!(v, 1897);
    }
}
