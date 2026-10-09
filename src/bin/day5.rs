use std::fs;

fn check_fresh(ranges: &Vec<Vec<i64>>, ingredients: &Vec<i64>) -> i64{
    let mut n = 0;

    for i in ingredients{
        for range in ranges{
            if *i >= range[0] && *i <= range[1]{
                n += 1;
                break;
            }
        }
    }
    n
}

fn check_number_fresh(ranges: &mut Vec<Vec<i64>>) -> i64{
    let list = make_list_all(ranges);
    let mut sum = 0;

    for range in list{
        sum += range[1] - range[0] + 1;
    } 

    sum
}

fn make_list_all(ranges: &mut Vec<Vec<i64>>) -> Vec<Vec<i64>>{
    let mut consolidated = vec!();
    ranges.sort_by_key(|v| v[0]);

    while !ranges.is_empty(){
        let mut first = ranges.remove(0);
        let mut to_remove = vec!();

        if consolidated.iter().any(|v: &Vec<i64>| v.contains(&first[0])){
            continue
        }

        for (i, range) in (&mut *ranges).iter().enumerate(){
            if first[1] >= range[0] && first[1] <= range[1]{
                to_remove.push(i);
                first[1] = range[1];
            }else if range[0] >= first[0] && range[1] <= first[1]{
                to_remove.push(i);
            }
        }

        to_remove.reverse();
        for i in to_remove{
            ranges.remove(i);
        }

        consolidated.push(first);
    }
    consolidated
}

fn read_input(path: &str) -> (Vec<Vec<i64>>, Vec<i64>){
    let contents = fs::read_to_string(path).unwrap();
    let split: Vec<&str> = contents.split("\n\n").collect();

    let ranges: Vec<Vec<i64>> = split[0].lines().map(|s| s.split('-').map(|n| n.parse().unwrap()).collect::<Vec<i64>>()).collect();
    let values: Vec<i64> = split[1].lines().map(|n| n.parse().unwrap()).collect();

    return (ranges, values);
}

fn main(){
    let mut input = read_input("inputs/day5/day5.txt");
    println!("Part 1: {}", check_fresh(&input.0, &input.1));
    println!("Part 2: {}", check_number_fresh(&mut input.0));
}