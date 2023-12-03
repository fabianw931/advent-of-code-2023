use std::{collections::HashMap, fs::File};

advent_of_code::solution!(1);

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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(142));
    }
}
