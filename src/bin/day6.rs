use std::fs;

// Part 1
fn solve(problems: &Vec<Vec<String>>) -> i64{
    let t_problems = transpose(problems);
    let mut bigsum = 0;

    for mut problem in t_problems{
        let sign = problem.remove(problem.len()-1) == "*";
        
        let mut sum = if sign {1} else {0};

        for n in problem{
            if sign{
                sum *= n.parse::<i64>().unwrap();
            }else{
                sum += n.parse::<i64>().unwrap();
            }
        }
        bigsum += sum;
    }

    bigsum
}

fn transpose<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>>{
    let mut t = vec!();

    for i in 0..arr[0].len(){
        let mut v = vec!();
        for j in 0..arr.len(){
            v.push(arr[j][i].clone());
        }
        t.push(v);
    }

    t
} 

fn read_input(path: &str) -> Vec<Vec<String>>{
    let contents = fs::read_to_string(path).unwrap();
    let m: Vec<Vec<String>> = contents.lines().map(|l| l.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()).collect();

    return m;
}

// Part 2

fn solve2(lines: &Vec<String>) -> i64{
    let positions = get_positions(lines);
    let problems: Vec<Vec<String>> = lines.iter().map(|l| get_slices(&l, &positions)).collect();
    let t_problems = transpose(&problems);   
    let mut bigsum = 0; 

    for mut problem in t_problems{
        let sign = problem.remove(problem.len()-1).contains("*");
        let mut sum = if sign {1} else {0};

        let nums: Vec<i64> = transpose(&problem.iter()
                        .map(|s| s.chars().collect()).collect())
                        .iter().map(|n| n.iter().filter(|x| **x != ' ').collect::<String>().parse::<i64>().unwrap()).collect();

        for n in nums{
            if sign{
                sum *= n;
            }else{
                sum += n;
            }

        }
        bigsum += sum
    }
    bigsum
}

fn get_positions(lines: &Vec<String>) -> Vec<usize>{
    (0..lines[0].len()).filter(|&i| {
        lines.iter().all(|s| s.as_bytes().get(i) == Some(&b' '))
    }).collect()
}

fn get_slices<'a>(line: &String, positions: &Vec<usize>) -> Vec<String>{
    let mut last = positions[0];
    let mut v = vec!(line[0..last].to_string());    

    for p in positions.iter().skip(1){
        v.push(line[last+1..*p].to_string());
        last = *p;
    }
    v.push(line[last+1..].to_string());
    v
}

fn read_input_raw(path: &str) -> Vec<String>{
    fs::read_to_string(path).unwrap().lines().map(|s| s.to_string()).collect()
}

fn main(){
    let input = read_input("inputs/day6/day6.txt");
    println!("{}", solve(&input));

    let input2 = read_input_raw("inputs/day6/day6.txt");
    println!("{}", solve2(&input2));
}