use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).ok();
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let sum: usize = re
            .captures_iter(&*input)
            .map(|cap| {
                let x: usize = cap[1].parse().unwrap();
                let y: usize = cap[2].parse().unwrap();
                x * y
            })
            .sum();
        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).ok();
        let search = vec!["don't()", "do()"];
        let mut i = 0;
        let mut start = 0;
        let mut trimmed_input = String::new();
        // println!("{input}");
        while start < input.len() {
            match input[start..].find(search[i]) {
                Some(index) => {
                    if i == 0 {
                        // push everything before "don't()"
                        trimmed_input.push_str(&input[start..start + index]);
                    }
                    start += index + search[i].len();
                }
                None => {
                    if i == 0 { // don't()
                        trimmed_input.push_str(&input[start..]);
                    }
                    break
                }
            }
            i = (i + 1) % 2
        }

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let sum: usize = re
            .captures_iter(&*trimmed_input)
            .map(|cap| {
                let x: usize = cap[1].parse().unwrap();
                let y: usize = cap[2].parse().unwrap();
                x * y
            })
            .sum();
        Ok(sum)
    }

    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
