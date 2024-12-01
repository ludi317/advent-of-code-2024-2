use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let nums = get_nums(&line);
            left.push(nums[0]);
            right.push(nums[1]);
        }

        left.sort_unstable();
        right.sort_unstable();
        let sum: usize = left.iter()
            .zip(right.iter())
            .map(|(l,r)| (l-r).abs() as usize)
            .sum();
        Ok(sum)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut left: HashMap<usize, usize> = HashMap::new();
        let mut right: HashMap<usize, usize> = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            let nums = get_nums_usize(&line);
            *left.entry(nums[0]).or_insert(0) += 1;
            *right.entry(nums[1]).or_insert(0) += 1;
        }
        let mut sum: usize = 0;
        for (key, left_value) in left.iter() {
            if let Some(right_value) = right.get(key) {
                sum += key * left_value * right_value
            }
        }
        Ok(sum)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
