use std::ops::Range;

use advent_of_code::helpers::RangeExt;
use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = Option<(Range<u32>, Range<u32>)>> + '_ {
    input.lines().map(|l| {
        let (elf_a, elf_b) = l.split_once(',')?;

        let (start_a, end_a) = elf_a.split_once('-')?;
        let res_a = start_a.parse().ok()?..end_a.parse().ok()?;

        let (start_b, end_b) = elf_b.split_once('-')?;
        let res_b = start_b.parse().ok()?..end_b.parse().ok()?;

        Some((res_a, res_b))
    })
}

pub(crate) fn part_one(input: &str) -> Option<u32> {
    parse(input).fold_options(0, |acc, (a, b)| {
        if a.is_subset(&b) || b.is_subset(&a) {
            acc + 1
        } else {
            acc
        }
    })
}

pub(crate) fn part_two(input: &str) -> Option<u32> {
    parse(input).fold_options(
        0,
        |acc, (a, b)| {
            if a.has_intersection(&b) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
