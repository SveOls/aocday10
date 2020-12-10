use std::{
    time::Instant,
    fs::File,
    io::{BufRead, BufReader},
};


fn file_loader(filename: &str) -> Option<Vec<i64>> {
    let mut ret = Vec::new();
    let mut file = BufReader::new(File::open(filename).ok()?).lines();
    while let Some(Ok(line)) = file.next() {
        ret.push(line.parse().ok()?)
    }
    Some(ret)
}

fn main() {
    let inp = file_loader("input.txt").unwrap();
    let time = Instant::now();
    println!("--------------------\nPart A\n");
    part1(&inp);
    println!("Time: {:?}", time.elapsed());
    println!("--------------------\nPart B\n");
    part2(&inp);
    println!("Time: {:?}", time.elapsed());
}

fn part1(inp: &Vec<i64>) -> Option<()> {
    let mut one = 0;
    let mut three = 1;
    let mut test = inp.clone();
    test.sort();
    let mut prev = 0;
    let mut i = 0;
    while i < test.len() {
        if test[i] == prev + 1 {
            prev = test[i]; 
            one += 1;
            i += 1;
        } else if test[i] == prev + 3 {
            prev = test[i];
            three += 1;
            i += 1; 
        } else {
            panic!()
        }
    }
    println!("{}", one * three);
    Some(())
}

fn part2(inp: &Vec<i64>) -> Option<()> {
    let mut fin = 1;
    let mut test = inp.clone();
    test.push(0);
    test.push(test.iter().max()? + 3);
    test.sort();

    let mut prev = 0;
    let mut result = Vec::new();
    let mut temp = 1;
    let mut i = 1;
    while i < test.len() {
        if test[i] == prev + 1 {
            temp += 1;
        } else {
            result.push(temp);
            temp = 1;
        }
        prev = test[i];
        i += 1;
    }
    result.push(temp);
    for i in result {
        fin *= tribonacci(i);
    }
    println!("{}", fin);
    Some(())
}

fn tribonacci(stuff: i64) -> i64 {
    match stuff {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 2,
        _ => tribonacci(stuff - 1) + tribonacci(stuff - 2) + tribonacci(stuff - 3)
    }
}