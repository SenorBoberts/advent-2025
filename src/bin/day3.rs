use std::fs;

fn find_top(s: &str) -> (i64, usize){
    let mut top = 0;
    let mut ptop = 0;
    for (i, c) in s.as_bytes().iter().enumerate(){
        let c64 = *c as i64;

        if c64 > top{
            top = c64;
            ptop = i;
        }
    }

    return (top - 48, ptop);
}

fn solve(banks: &Vec<String>) -> i64{
    let mut sum = 0;

    for b in banks{
        let top = find_top(&b[..b.len()-1]);
        let stop = find_top(&b[top.1+1..]);

        sum += top.0 * 10 + stop.0;
    }
    sum
}

fn solve2(banks: &Vec<String>) -> i64{
    let mut sum = 0;

    for b in banks{
        sum += find12(b);
    }

    sum
}

fn find12(s: &str) -> i64{
    let mut vals: Vec<(i64, usize)> = vec!();
    for i in (0..=11).rev(){
        let range = if let Some(t) = vals.last(){
            t.1 + 1
        }else{
            0
        };
        let mut tup = find_top(&s[range..s.len()-i]);
        tup.1 += range;
        vals.push(tup);
    }

    let mut sum = 0;
    for (i, t) in vals.iter().enumerate(){
        sum += t.0 * 10_i64.pow(11-i as u32);
    }
    sum
}

fn read_input(path: &str) -> Vec<String>{
    let contents = fs::read_to_string(path).unwrap();
    return contents.split('\n').map(|s| s.to_string()).collect();
}

fn main(){
    let input = read_input("inputs/day3/day3.txt");
    println!("Part 1: {}", solve(&input));
    println!("Part 2: {}", solve2(&input));
}