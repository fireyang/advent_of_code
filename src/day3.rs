
mod day3 {
    use std::collections::HashMap;

    fn get_map(vec: &Vec<String>) -> (i32, i32, HashMap<(usize, usize), char>){
        // let mut map = HashMap::new();
        let mut w:usize = 0;
        // let mut h = 0;
        let v: Vec<Vec<((usize, usize), char)>> = vec
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let clist = value.chars().enumerate()
                    .filter(|(_, c)| *c=='#')
                    .map(|(idx2, c)| ((idx2, idx), c)).collect();
                if w < value.len(){
                    w = value.len();
                }
                clist
            })
            .collect();
        let h = vec.len();
        let map: HashMap<(usize, usize), char> = v.iter().flatten().cloned().collect();
        // println!("{:?}",map);
        // for (i, it) in v.iter() {
        //     for (j, c) in it.iter() {
        //         if c == '#':
        //             map.insert(i*100 + j, c)
        //     }
        // }
        (w as i32, h as i32, map)
    }


    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>, mov: (i32, i32)) -> i32 {
        // let mut start_post = (1,1);
        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let (w, h, map) = get_map(&vec);
        // println!("{:?}",(w, h, map.len()));
        let (mx, my) = mov;
        let mut count = 0;
        while y <= h{
            x += mx;
            x = x % w;
            y+= my;
            let key = &(x as usize, y as usize);
            // println!("pos:{:?}", ( x, y, map.get(key)));
            if let Some(_) = map.get(key){
                count +=1;
            }
        }

        // let mut count =  0;
        // for it in v.iter(){
        //     let (min, max, c, s) = it;
        //     let v = s.chars().enumerate().filter(|(_, x)| x == c).count() as i32;
        //     if v >= *min && v <= *max{
        //         count += 1;
        //     }
        // }
        count
    }

    // #[allow(dead_code)]
    // pub fn part2(vec: Vec<String>) -> i32 {
    //     let v = get_inputs(&vec);
    //     let mut count =  0;
    //     for it in v.iter(){
    //         let (min, max, c, s) = it;
    //         let s_char = s.chars().nth((*min -1) as usize).unwrap();
    //         let e_char = s.chars().nth((*max -1) as usize).unwrap();
    //         if (&s_char == c || &e_char == c) && s_char != e_char{
    //             // println!("{:?}", (it, s_char, e_char));
    //             count += 1;
    //         }
    //     }
    //     count
    // }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day3_part1() {
        let list = common::parse_from_file("./data/day3_part1.txt");
        // let list = common::parse_from_file("./data/day3_test.txt");
        let result = day3::part1(list.unwrap(), (3, 1));
        assert_eq!(result, 216);
    }

    #[test]
    fn day3_part2() {
        // let list = common::parse_from_file("./data/day3_test.txt");
        let list = common::parse_from_file("./data/day3_part1.txt");
        let move_list = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
        let mut count:i64 = 1;
        for it in move_list{
            let v = day3::part1(list.clone().unwrap(), it);
            // println!("out:{:?}", (it, v, count));
            count *= v as i64;
        }
        assert_eq!(count, 6708199680);
    }
}
