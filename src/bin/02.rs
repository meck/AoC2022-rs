#![allow(clippy::identity_op)]
use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = Option<(char, char)>> + '_ {
    input.lines().map(|line| {
        let abc = line.chars().next()?;
        let xyz = line.chars().nth(2)?;
        Some((abc, xyz))
    })
}

pub(crate) fn part_one(input: &str) -> Option<u32> {
    let mut inputs = parse(input);
    inputs.fold_options(0, |tot_score, hands| {
        let round_score = match hands {
            // Type score + Result score
            ('A', 'X') => 1 + 3,
            ('B', 'X') => 1 + 0,
            ('C', 'X') => 1 + 6,

            ('A', 'Y') => 2 + 6,
            ('B', 'Y') => 2 + 3,
            ('C', 'Y') => 2 + 0,

            ('A', 'Z') => 3 + 0,
            ('B', 'Z') => 3 + 6,
            ('C', 'Z') => 3 + 3,
            _ => panic!("Invalid Input: {hands:?}"),
        };
        tot_score + round_score
    })
}

pub(crate) fn part_two(input: &str) -> Option<u32> {
    let mut inputs = parse(input);
    inputs.fold_options(0, |tot_score, hands| {
        let round_score = match hands {
            // Type score + Result score
            ('A', 'X') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('C', 'X') => 2 + 0,

            ('A', 'Y') => 1 + 3,
            ('B', 'Y') => 2 + 3,
            ('C', 'Y') => 3 + 3,

            ('A', 'Z') => 2 + 6,
            ('B', 'Z') => 3 + 6,
            ('C', 'Z') => 1 + 6,
            _ => panic!("Invalid Input: {hands:?}"),
        };
        tot_score + round_score
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
