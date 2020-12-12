mod day12 {
    use std::cell::RefCell;

    #[derive(Debug, Clone)]
    enum Dirt{
        EAST =0,
        NORTH=1,
        WEST=2,
        SOUTH=3,
    }

    #[derive(Debug)]
    struct Ship{
        face: Dirt,
        x: i32,
        y: i32,
    }

    fn get_dir(v: i32) ->Dirt{
        let v = v%4;
        match v{
            0 => Dirt::EAST,
            1 => Dirt::NORTH,
            2 => Dirt::WEST,
            _ => Dirt::SOUTH,
        }
    }

    fn get_dir_idx(dirt: Dirt) ->i32{
        match dirt{
            Dirt::EAST => 0,
            Dirt::NORTH => 1,
            Dirt::WEST =>2,
            _ => 3,
        }
    }

    fn turn_dir(dirt:Dirt, turn: i32) -> Dirt{
        get_dir(get_dir_idx(dirt) + turn)
    }

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let ship = RefCell::new(Ship{face: Dirt::EAST, x:0,y:0});
        vec.iter()
            .for_each(|it| {
                // println!("src:{:?}", it);
                let (left, right) = it.split_at(1);
                let v:i32 = right.parse().unwrap();
                let mut ship = ship.borrow_mut();
                match left{
                    "N" => ship.y -= v,
                    "S" => ship.y += v,
                    "W" => ship.x -= v,
                    "E" => ship.x += v,
                    "L" =>{
                        let turn:i32 = match v{
                            90 => 1,
                            180 => 2,
                            _=> 3,
                        };
                        ship.face = turn_dir(ship.face.clone() , turn);
                    },
                    "R" =>{
                        let turn:i32 = match v{
                            90 => -1,
                            180 => -2,
                            _=> -3,
                        };
                        ship.face = turn_dir(ship.face.clone() , turn+4);
                    },
                    "F" =>{
                        match ship.face{
                            Dirt::NORTH => ship.y -= v,
                            Dirt::SOUTH => ship.y += v,
                            Dirt::WEST => ship.x -= v,
                            _ => ship.x += v,
                        }
                    },
                    _ => (),
                };
                // println!("m:{:?}", ship);
            });
            println!("{:?}", ship);
            let k =ship.borrow().x.abs() + ship.borrow().y.abs();
            k
    }

    fn waypoint_turn(v: (i32, i32), turn: i32) -> (i32, i32) {
        match turn {
            -1 => (-v.1, v.0),
            -2 => (-v.0, -v.1),
            -3 => (v.1, -v.0),
            1 => (v.1, -v.0),
            2 => (-v.0, -v.1),
            3 => (-v.1, v.0),
            _ => (v.0, v.1),
        }
    }


    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> i32 {
        let mut ship = (0,0);
        let mut waypoint = (10, -1);
        vec.iter()
            .for_each(|it| {
                // println!("src:{:?}", it);
                let (left, right) = it.split_at(1);
                let v:i32 = right.parse().unwrap();
                match left{
                    "N" => waypoint.1 -= v,
                    "S" => waypoint.1 += v,
                    "W" => waypoint.0 -= v,
                    "E" => waypoint.0 += v,
                    "L" =>{
                        let turn:i32 = match v{
                            90 => 1,
                            180 => 2,
                            _=> 3,
                        };
                        waypoint = waypoint_turn(waypoint, turn);
                    },
                    "R" =>{
                        let turn:i32 = match v{
                            90 => -1,
                            180 => -2,
                            _=> -3,
                        };
                        waypoint = waypoint_turn(waypoint, turn);
                    },
                    "F" =>{
                        ship = (ship.0+waypoint.0 * v, ship.1+waypoint.1 * v);
                    },
                    _ => (),
                };
                // println!("m:{:?}", (ship,waypoint));
            });
            println!("{:?}", ship);
            let k =ship.0.abs() + ship.1.abs();
            k
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day12_part1() {
        // let list = common::parse_from_file("./data/day12_test.txt");
        let list = common::parse_from_file("./data/day12_part1.txt");
        let v = day12::part1(Box::new(list.unwrap()));
        assert_eq!(v, 998);
    }

    #[test]
    fn day12_part2() {
        // let list = common::parse_from_file("./data/day12_test.txt");
        let list = common::parse_from_file("./data/day12_part1.txt");
        let v = day12::part2(Box::new(list.unwrap()));
        assert_eq!(v, 71586);
    }
}
