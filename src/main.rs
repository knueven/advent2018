use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::collections::HashSet;

fn main() -> Result<()> {
    // part 1
    let path = Path::new("C:\\Users\\Andrew\\IdeaProjects\\aoc-1\\src\\input.txt");
    let mut i = 0;
    //while calculating the part 1 sum hold the input values in memory for later
    let mut vec = Vec::new();
    let f = File::open(path).expect("file not found");
        for line in BufReader::new(f).lines() {
            let n = line.unwrap();
            let n = n.trim();
            let atoi = n.parse::<i32>().unwrap();
            vec.push(atoi);
            i += atoi;
        }
    println!("Part 1: {:?}", i);

    let mut numset = HashSet::new();
    let mut c = 0;
    numset.insert(c);
    for i in vec.iter().cycle() {
        c += i;
        if !numset.insert(c) {
            break;
        }
    }
    println!("Part 2: {}", c);
    Ok(())

    //println!("With text:\n{}", contents);
}
