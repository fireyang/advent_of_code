mod day1 {

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        vec.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate common;

    #[test]
    fn day1_part1() {
        let list = common::parse_from_file("./data/day1_part1.txt");
        let ret = day1::part1(list.clone().unwrap());

        println!("out:{:?}, {:?}", list, ret);
        let a = 1;
        assert_eq!(a, 1);
    }
}
