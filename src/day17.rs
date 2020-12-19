mod day17 {
    use std::collections::HashMap;

    type Position = (i32, i32, i32);
    type Range = (i32, i32);

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let mut grid: HashMap<Position, i32> = HashMap::new();
        vec.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let v = match c {
                    '.' => 0,
                    _ => 1,
                };
                if v > 0 {
                    grid.insert((x as i32, y as i32, 0), v);
                }
            });
        });
        println!("{:?}", grid);
        // print_grid(&grid);
        for i in 0..6 {
            println!("turn {}", i);
            grid = turn_one(&grid);
            // print_grid(&new_grid);
        }
        println!("out {}", grid.len());
        grid.len() as i32
    }

    fn turn_one(grid: &HashMap<Position, i32>) -> HashMap<Position, i32> {
        let mut new_grid: HashMap<Position, i32> = HashMap::new();
        let (x_r, y_r, z_r) = get_grid_range(&grid);
        for z in z_r.0..z_r.1 + 3 {
            for y in y_r.0..y_r.1 + 3 {
                for x in x_r.0..x_r.1 + 3 {
                    let pos = (x - 1 as i32, y - 1 as i32, z - 1 as i32);
                    let num = get_active_num(&grid, &pos);
                    // println!("{:?}", (pos, num));
                    if let Some(_) = grid.get(&pos) {
                        if num == 2 || num == 3 {
                            new_grid.insert(pos, 1);
                        }
                    } else if num == 3 {
                        new_grid.insert(pos, 1);
                    }
                }
            }
        }
        new_grid
    }

    fn get_grid_range(grid: &HashMap<Position, i32>) -> (Range, Range, Range) {
        let mut x_range = (i32::MAX, i32::MIN);
        let mut y_range = (i32::MAX, i32::MIN);
        let mut z_range = (i32::MAX, i32::MIN);
        grid.iter().for_each(|(k, _v)| {
            x_range = get_range(x_range, k.0);
            y_range = get_range(y_range, k.1);
            z_range = get_range(z_range, k.2);
        });
        (x_range, y_range, z_range)
    }

    fn get_active_num(grid: &HashMap<Position, i32>, pos: &Position) -> i32 {
        let neighbors = get_neighbor(pos);
        let mut count = 0;
        for it in neighbors.iter() {
            // if pos == &(1,0,0) {
            //     println!("ner:{:?}", (it, grid.get(it)));
            // }
            if let Some(_) = grid.get(it) {
                count += 1;
            }
        }
        count
    }

    fn get_neighbor(pos: &Position) -> Vec<Position> {
        let mut list = Vec::with_capacity(26);
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let x = pos.0 + i;
                    let y = pos.1 + j;
                    let z = pos.2 + k;
                    let new_pos = (x as i32 - 1, y as i32 - 1, z as i32 - 1);
                    if &new_pos != pos {
                        list.push(new_pos);
                    }
                }
            }
        }
        list
    }

    #[allow(dead_code)]
    fn print_grid(grid: &HashMap<Position, i32>) {
        let (x_range, y_range, z_range) = get_grid_range(&grid);
        for z in z_range.0..z_range.1 + 1 {
            println!("z={:?}", z);
            for y in y_range.0..y_range.1 + 1 {
                let mut s = String::from("");
                for x in x_range.0..x_range.1 + 1 {
                    if let Some(_) = grid.get(&(x, y, z)) {
                        s.push('#');
                    } else {
                        s.push('.');
                    }
                }
                println!("{}", s);
            }
        }
    }

    fn get_range(mut range: (i32, i32), val: i32) -> (i32, i32) {
        if range.0 > val {
            range.0 = val;
        }
        if range.1 < val {
            range.1 = val;
        }
        // println!("{:?}, {:?}", range,val);
        range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day17_part1() {
        // let list = common::parse_from_file("./data/day17_test.txt");
        let list = common::parse_from_file("./data/day17_part1.txt");
        let v = day17::part1(list.unwrap());
        assert_eq!(v, 293);
    }
}
