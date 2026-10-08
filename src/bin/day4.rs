use std::{fs, str};

fn count_rolls(map: &Vec<Vec<char>>) -> i32{
    let mut sum = 0;

    for i in 0..map.len(){
        for j in 0..map[0].len(){
            if num_adacent(i, j, map) < 4{
                sum += 1;
            }
        }
    }
    sum
}

fn num_adacent(x: usize, y: usize, map: &Vec<Vec<char>>) -> i32{
    if map[x][y] == '@'{
        let mut sum = 0;

        for dx in -1..=1{
            for dy in -1..=1{
                if dx == 0 && dy == 0{
                    continue 
                }
                
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < map.len() as isize && ny >= 0 && ny < map.len() as isize && map[nx as usize][ny as usize] == '@'{
                    sum += 1
                }

            }
        }
        return sum;
    }
    8
}

fn count_and_delete_rolls(map: &mut Vec<Vec<char>>) -> i32{
    let mut sum = 0;

    for i in 0..map.len(){
        for j in 0..map[0].len(){
            delete_self(i, j, &mut sum, map);
        }
    }

    sum
}

fn delete_self(x: usize, y: usize, sum: &mut i32,map: &mut Vec<Vec<char>>) -> i32{
    if num_adacent(x, y, map) < 4{
        map[x][y] = '.';
        *sum += 1;

        for dx in -1..=1{
            for dy in -1..=1{
                

                if dx == 0 && dy == 0{
                    continue
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 && nx < map.len() as isize && ny < map[0].len() as isize{
                    delete_self(nx as usize, ny as usize, sum, map);
                }

            }
        }
    }

    *sum
}

fn read_input(path: &str) -> Vec<Vec<char>>{
    fs::read_to_string(path).unwrap().lines().map(|s| s.chars().collect()).collect()
}

fn main(){
    let mut input = read_input("inputs/day4/day4.txt");
    println!("Part 1: {}", count_rolls(&input));
    println!("Part 2: {}", count_and_delete_rolls(&mut input));
}