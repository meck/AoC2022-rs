use std::{collections::BinaryHeap, iter::Iterator};

fn parse(input: &str) -> BinaryHeap<u32> {
    input
        .split("\n\n")
        .map(|x| x.lines().filter_map(|i| i.parse::<u32>().ok()).sum())
        .collect()
}

pub(crate) fn part_one(input: &str) -> Option<u32> {
    parse(input).pop()
}

pub(crate) fn part_two(input: &str) -> Option<u32> {
    let mut inv = parse(input);
    Some(inv.pop()? + inv.pop()? + inv.pop()?)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
