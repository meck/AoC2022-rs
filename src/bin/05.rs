/// Number to move, from stack, destination stack
type Move = (usize, usize, usize);

#[derive(Debug, Default)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn push(&mut self, label: char, dest: usize) {
        while self.stacks.len() <= dest {
            self.stacks.push(Vec::new());
        }
        self.stacks[dest].push(label);
    }

    fn move_one(&mut self, (n, from, dest): Move) {
        for _ in 0..n {
            let c = self.stacks[from].pop().expect("Moved from empty stack");
            self.stacks[dest].push(c);
        }
    }

    fn move_multiple(&mut self, (n, from, dest): Move) {
        let range = self.stacks[from].len() - n..;
        let res: Vec<char> = self.stacks[from].drain(range).collect();
        self.stacks[dest].extend(res);
    }

    fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().unwrap_or(&' '))
            .collect()
    }
}

fn parse(input: &str) -> Option<(Stacks, Vec<Move>)> {
    let mut input_parts = input.split("\n 1 ");

    // Stacks
    let mut stacks: Stacks = Stacks::default();
    let stacks_layout = input_parts.next()?.lines().rev();
    for layer in stacks_layout {
        layer
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(dest, label)| {
                if label.is_ascii_uppercase() {
                    stacks.push(label, dest);
                }
            });
    }

    // Moves
    let mut moves: Vec<Move> = vec![];
    let moves_line = input_parts.next()?.lines().skip(2);
    for a_move in moves_line {
        let mut m2 = a_move.split_ascii_whitespace();
        let amount: usize = m2.nth(1)?.parse().ok()?;
        let from: usize = m2.nth(1)?.parse().ok()?;
        let dest: usize = m2.nth(1)?.parse().ok()?;
        // 0 index
        moves.push((amount, from - 1, dest - 1));
    }

    Some((stacks, moves))
}

pub(crate) fn part_one(input: &str) -> Option<String> {
    let (mut st, mv) = parse(input)?;
    for m in mv {
        st.move_one(m);
    }
    Some(st.tops())
}

pub(crate) fn part_two(input: &str) -> Option<String> {
    let (mut st, mv) = parse(input)?;
    for m in mv {
        st.move_multiple(m);
    }
    Some(st.tops())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
