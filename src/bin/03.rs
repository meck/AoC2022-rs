use std::collections::HashSet;

use itertools::Itertools;

fn ascii_score(c: char) -> Option<u32> {
    match c {
        c if c.is_ascii_lowercase() => Some(c as u32 - 96),
        c if c.is_ascii_uppercase() => Some(c as u32 - 38),
        _ => None,
    }
}

/// Get an iterator over all common chars in strings
fn strings_intersection<'a>(
    strings: impl Iterator<Item = &'a str>,
) -> Option<impl Iterator<Item = char>> {
    let mut lines_hash_sets = strings.map(|line| line.chars().collect::<HashSet<_>>());
    let mut result: HashSet<char> = lines_hash_sets.next()?;
    for line in lines_hash_sets {
        result = result.intersection(&line).copied().collect();
    }
    Some(result.into_iter())
}

pub(crate) fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (string1, string2) = line.split_at(line.len() / 2);

            // Reference hashset
            let set: HashSet<char> = string1.chars().collect();

            let priority: Option<u32> = string2
                .chars()
                .filter(|c| set.contains(c))
                .unique()
                .map(ascii_score)
                .sum();

            priority
        })
        .sum()
}

pub(crate) fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elf_group| {
            strings_intersection(elf_group)?
                .map(ascii_score)
                .sum::<Option<u32>>()
        })
        .sum()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
