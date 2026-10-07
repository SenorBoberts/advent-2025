use std::fs;

// PART 1
fn count_zeros_part1(arr: &Vec<i32>)-> i32{
    let mut pos: i32 = 50;
    let mut count: i32 = 0;

    for x in arr{
        pos = overflow(pos + x);

        if pos == 0{
            count += 1
        } 
    }

    return count;
}

fn overflow(x: i32) -> i32{
    if x >= 100{
        return x % 100
    }
    if x < 0{
        if x % 100 == 0{
            return 0;
        }else{
            return 100 + x % 100;
        }
    }
    x
}

// PART 2
fn count_zeros_part2(arr: &Vec<i32>) -> i32{
    let mut pos: i32 = 50;
    let mut count: i32 = 0;

    for x in arr{

        if pos + x >= 100{
            count += (pos + x) / 100;
        }

        if pos + x < 0{
            if pos == 0{
                count += ((pos + x) / 100).abs();
            }else{
                count += 1 + ((pos + x) / 100).abs();
            }
        }

        if pos + x == 0{
            count += 1;
        }
        
        pos = overflow(pos + x);
    }


    return count;
}

fn readfile(p: &str) -> Vec<i32>{
    let contents = fs::read_to_string(p).unwrap();
    let vals = contents.split("\n");
    let mut ns = vec!();

    for v in vals{
        let mut n: i32 = v[1..].parse().unwrap();

        if &v[0..1] == "L"{
            n = n * - 1;
        }

        ns.push(n);
    }

    return ns;
}

pub fn day1(){
    let vals = readfile("inputs/day1/day1.txt");
    let zeros = count_zeros_part1(&vals);

    println!("Part 1: {}", zeros);

    let zeros2 = count_zeros_part2(&vals);

    println!("Part 2: {}", zeros2);
}