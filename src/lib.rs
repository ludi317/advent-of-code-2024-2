pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions

pub fn get_nums_usize(s: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    let mut num: usize = 0;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + (c.to_digit(10).unwrap() as usize);
            num_found = true;
        } else if num_found {
            nums.push(num);
            num  = 0;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num);
    }
    nums
}

// parses a sequence of numbers
pub fn get_nums(s: &str) -> Vec<isize> {
    let mut nums: Vec<isize> = vec![];
    let mut num: isize = 0;
    let mut sign = 1;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + c.to_digit(10).unwrap() as isize;
            num_found = true;
        } else if c == '-' {
            sign = -1;
        } else if num_found {
            nums.push(num * sign);
            num = 0;
            sign = 1;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num * sign);
    }
    nums
}

pub fn abs(x: isize) -> usize {
    if x < 0 {
        -x as usize
    } else {
        x as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }

    #[test]
    fn test_get_nums() {
        let s = "4 27 78 180 375 742  0";
        let v = vec![4, 27, 78, 180, 375, 742, 0];
        assert_eq!(get_nums_usize(s), v);
    }
}
