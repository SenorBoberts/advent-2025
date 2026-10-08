use std::collections::HashSet;
use std::fs;

// PART 1
fn check_range(ranges: &Vec<Vec<i64>>) -> i64{
    let mut sum = 0;
    for r in ranges{
        for i in r[0]..=r[1]{
            if is_valid(i){
                sum += i;
            }
        }
    }
    sum
}

fn is_valid(x: i64) -> bool{
    let s = x.to_string();

    if !s.len() % 2 == 0{
        //Number cannot be symetric
        return false;
    }

    if s[0..s.len()/2] == s[s.len()/2..]{
        return true;
    }
    false
}

// PART 2

fn check_ranges_part2(ranges: &Vec<Vec<i64>>) -> i64{
    let mut sum = 0;
    for r in ranges{
        for i in r[0]..=r[1]{
            if check_num(&i.to_string()){
                sum += i
            }
        }
    }
    sum
}

fn check_num(n: &str) -> bool{
    for i in 1..=n.len() / 2{
        if is_valid_window(n, i){
            return true;
        }
    }
    false
}

fn is_valid_window(s: &str, size: usize) -> bool{

    //number cannot be divided into range    
    if s.len() % size != 0{
        return false;
    }

    let hs: HashSet<&[u8]> = s.as_bytes().chunks(size).collect();
    
    return hs.len() == 1;
    
}

fn read_input(path: &str) -> Vec<Vec<i64>>{
    let contents = fs::read_to_string(path).unwrap();
    let ranges = contents.split(',');
    return ranges.map(|r| {
        r.split('-').map(|x| x.parse().unwrap()).collect::<Vec<i64>>()
    }).collect();
}

fn main(){
    let input = read_input("inputs/day2/day2.txt");
    println!("Part 1: {}", check_range(&input));
    println!("Part 2: {}", check_ranges_part2(&input));
}