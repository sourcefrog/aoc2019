use mbp_aoc2019::Matrix;

const BUG: char = '#';
const EMPTY: char = '.';

pub fn main() {
    println!("24a: {}", solve_a());
}

fn solve_a() -> isize {
    solve_type_a(&std::fs::read_to_string("input/input24.txt").unwrap())
}

fn solve_type_a(s: &str) -> isize {
    let mut states = vec![Matrix::from_string_lines(s)];
    loop {
        let next = next_state(states.last().unwrap());
        println!("{}", next.to_string_lines());
        if states.contains(&next) {
            return biodiversity(&next);
        }
        states.push(next);
    }

    // 36742190 is too high?
}

fn next_state(m: &Matrix<char>) -> Matrix<char> {
    let mut next = Matrix::new(5, 5, '?');
    for p1 in m.iter_points() {
        let ch1 = m[p1];
        let bug_count = m
            .neighbors4(p1)
            .iter()
            .filter(|(_p2, ch2)| **ch2 == BUG)
            .count();
        next[p1] = if bug_count == 1 || (ch1 == EMPTY && bug_count == 2) {
            BUG
        } else {
            EMPTY
        };
    }
    next
}

fn biodiversity(m: &Matrix<char>) -> isize {
    // iter_points goes row at a time from top to bottom
    m.iter_points()
        .map(|p| m[p])
        .enumerate()
        .filter(|(_i, ch)| *ch == BUG)
        .map(|(i, _ch)| 1 << i)
        .sum()
}

#[cfg(test)]
const EXAMPLE: &str = "\
....#
#..#.
#..##
..#..
#....
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        assert_eq!(solve_type_a(EXAMPLE), 2_129_920);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 18_371_095);
    }
}
