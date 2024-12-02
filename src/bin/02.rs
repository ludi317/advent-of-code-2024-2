use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let count = reader
            .lines()
            .filter_map(|line_result| {
                line_result.ok()
            })
            .filter(|line| {
                let nums = get_nums(line);
                let first_diff = nums[0] - nums[1];

                let valid_range = match first_diff {
                    1..=3 => 1..=3,
                    -3..=-1 => -3..=-1,
                    _ => return false,
                };

                nums[1..]
                    .windows(2)
                    .map(|window| window[0] - window[1])
                    .all(|x| valid_range.contains(&x))
            })
            .count();

        Ok(count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut skipped_nums: Vec<isize> = vec![0isize; 7];
        let count = reader
            .lines()
            .filter_map(|line_result| {
                line_result.ok()
            })
            .filter(|line| {
                let nums = get_nums(line);
                for skip_idx in 0..nums.len() {
                    for i in 0..skip_idx {
                        skipped_nums[i] = nums[i];
                    }
                    for i in skip_idx + 1..nums.len() {
                        skipped_nums[i - 1] = nums[i]
                    }
                    let first_diff = skipped_nums[0] - skipped_nums[1];

                    let valid_range = match first_diff {
                        1..=3 => 1..=3,
                        -3..=-1 => -3..=-1,
                        _ => continue,
                    };
                    if skipped_nums[1..nums.len() - 1]
                        .windows(2)
                        .map(|window| window[0] - window[1])
                        .all(|x| valid_range.contains(&x))
                    {
                        return true;
                    }
                }
                false
            })
            .count();

        Ok(count)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
