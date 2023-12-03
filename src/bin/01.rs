use std::collections::HashMap;

advent_of_code::solution!(1);

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            // only get first and last digit and then sum
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            let first = digits.first().unwrap_or(&0);
            let last = digits.last().unwrap_or(&0);
            let sum_as_int: u32 = format!("{}{}", first, last).parse().unwrap_or(0);
            sum_as_int
        })
        .sum::<u32>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut vec: Vec<u32> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                vec.push(c.to_digit(10).unwrap());
            } else {
                for (j, d) in DIGITS.iter().enumerate() {
                    if line.as_bytes()[i..].starts_with(d.as_bytes()) {
                        vec.push(j as u32 + 1);
                        break;
                    }
                }
            }
        }
        sum += vec.first().unwrap() * 10 + vec.last().unwrap();
    }

    sum.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
