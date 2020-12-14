mod day14 {
    use std::collections::HashMap;

    fn get_mask_result(mask: (u64, u64), v: u64)-> u64{
        let v1 = v & mask.0;
        let v2 = v1 | mask.1;
        // println!("11:v_0:{:#b} v_1:{:#b}", mask.0, mask.1);
        // println!("22:v_0:{:#b} v_1:{:#b}", v1, v2);
        v2
    }

    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> u64 {
        // let mut mask:String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
        let mut mask = (u64::MAX, 0);
        let mut map:HashMap<u32, u64> = HashMap::new();
        vec.iter().for_each(|s| {
            let list:Vec<&str> =  s.splitn(2, "=").collect();
            let pre = list[0].trim();
            match pre{
                "mask" => {
                    let m = list[1].trim().to_string().clone();
                    let mut v_1 :u64 = 0;
                    let mut v_0 :u64 = 0;
                    for c in m.chars(){
                        match c {
                            '1' =>{
                                v_1 = v_1 << 1 | 1;
                                v_0 = v_0 << 1 | 1;
                            },
                            '0' =>{
                                v_1 = v_1 << 1;
                                v_0 = v_0 << 1;
                            },
                            _ => {
                                v_1 = v_1 << 1;
                                v_0 = v_0 << 1 | 1;
                            }
                        }
                    }
                    // println!("v_0:{:#b}\nv_1:{:#b}", v_0, v_1);
                    mask = (v_0, v_1);
                },
                v => {
                    let memory:u32 = v[4..v.len()-1].parse().unwrap();
                    let v:u64 = list[1].trim().parse().unwrap();
                    let out = get_mask_result(mask, v);
                    map.insert(memory, out);
                    // println!("{:?}", (memory, v, out));
                }

            }

        });

        // println!("{:?}", map);
        let mut all = 0;
        for (_k, v) in map.iter(){
            all += v;
        }
        all
    }


    fn get_mask_addr(memory:u64, mask:&Vec<char> ) -> Vec<u64>{
        // let count = mask.iter().filter(|&c| c == &'X').count();
        let mut v = memory;
        mask.iter().enumerate().filter(|(_, &c)|c=='1').
            for_each(|(idx, _)|{
                v = v | (1 << idx);
            });
        let mut list = vec![v];
        mask.iter().enumerate().filter(|(_, &c)|c=='X').
            for_each(|(idx, _)|{
                for i in 0..list.len(){
                        list[i] = list[i] | (1 << idx);
                        list.push(list[i] & (!(1 << idx)));
                }
            });
        list
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> u64 {
        // let mut mask :[char; 36] = ['X'; 36];
        let mut mask  =vec![];
        let mut map:HashMap<u64, u64> = HashMap::new();
        vec.iter().for_each(|s| {
            let list:Vec<&str> =  s.splitn(2, "=").collect();
            let pre = list[0].trim();
            match pre{
                "mask" => {
                    mask = list[1].trim().chars().collect();
                    mask.reverse();
                },
                v => {
                    let memory:u64 = v[4..v.len()-1].parse().unwrap();
                    let v:u64 = list[1].trim().parse().unwrap();
                    let memory_list = get_mask_addr(memory, &mask);
                    // println!("{:#b}", memory);
                    for it in memory_list.iter(){
                        // println!("  {:#b}", it);
                        map.insert(it.clone(), v);
                    }
                    // println!("{:?}", (memory, v, out));
                }

            }
        });
        let mut all = 0;
        for (_k, v) in map.iter(){
            all += v;
        }
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day14_part1() {
        // let list = common::parse_from_file("./data/day14_test.txt");
        let list = common::parse_from_file("./data/day14_part1.txt");
        let v = day14::part1(Box::new(list.unwrap()));
        assert_eq!(v, 8566770985168);
    }

    #[test]
    fn day14_part2() {
        // let list = common::parse_from_file("./data/day14_test.txt");
        let list = common::parse_from_file("./data/day14_part1.txt");
        let v = day14::part2(Box::new(list.unwrap()));
        assert_eq!(v, 4832039794082);
    }

}
