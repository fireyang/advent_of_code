mod day8 {

    // use std::collections::HashMap;
    use std::collections::HashSet;


    #[allow(dead_code)]
    pub fn part1(vec: Box<Vec<String>>) -> i32 {
        let v:Vec<_> = _parse(vec);
        let mut set: HashSet<_> = HashSet::new();
        let mut cur = 0;
        let mut result = 0;
        while !set.contains(&cur){
            set.insert(cur);
            if let Some((op, v)) = v.get(cur){
                cur = match op.as_str(){
                    "nop" => cur + 1,
                    "acc" => {result +=v; cur + 1},
                    "jmp" =>  (cur as i32 + *v) as usize,
                    _ => cur,
                }
            }
        }
        result
    }

    fn _parse(vec: Box<Vec<String>>) -> Vec<(String, i32)>{
        let v = vec.iter()
            .map(|it |{
                let v:Vec<&str> = it.splitn(2," ").collect();
                let num = v[1].to_string().parse::<i32>().unwrap();
                (v[0].trim().to_string(), num)
            })
            .collect();
        v
    }

    fn _run(vec: Vec<(String, i32)>) -> (bool, i32){
        let mut set: HashSet<_> = HashSet::new();
        let mut cur = 0;
        let mut result = 0;
        let mut ok = false;
        while !set.contains(&cur){
            set.insert(cur);
            if let Some((op, v)) = vec.get(cur){
                cur = match op.as_str(){
                    "nop" => cur + 1,
                    "acc" => {result +=v; cur + 1},
                    "jmp" =>  (cur as i32 + *v) as usize,
                    _ => cur,
                }
            }else{
                ok = true;
            }
        }
        (ok, result)
    }

    #[allow(dead_code)]
    pub fn part2(vec: Box<Vec<String>>) -> i32 {
        let v:Vec<_> = _parse(vec);
        for i in 0..v.len(){
            let mut k:Vec<_> = v.clone();
            let (op, v) = k[i].clone();
            k[i] = match op.as_str(){
                "jmp" => ("nop".to_string(), v),
                "nop" => ("jmp".to_string(), v),
                _ => (op, v),
            };
            // println!("{:?}", k);
            let (ok, ret) = _run(k);
            if ok{
                // println!("{:?}", (i, ret));
                return ret;
            }
        }
        0
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day8_part1() {
        // let list = common::parse_from_file("./data/day8_test.txt");
        let list = common::parse_from_file("./data/day8_part1.txt");
        let v = day8::part1(Box::new(list.unwrap()));

        assert_eq!(v, 2080);
    }

    #[test]
    fn day8_part2() {
        let list = common::parse_from_file("./data/day8_part1.txt");
        // let list = common::parse_from_file("./data/day8_test.txt");
        let v = day8::part2(Box::new(list.unwrap()));

        assert_eq!(v, 2477);
    }

}
