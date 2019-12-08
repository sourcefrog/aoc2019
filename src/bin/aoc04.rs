pub fn main() {
    println!("04a: {}", solve_a());
}

/// Return digit d (counting from the left) of i.
pub fn digit(i: usize, d: usize) -> usize {
    (match d {
        0 => i / 100_000,
        1 => i / 10_000,
        2 => i / 1_000,
        3 => i / 100,
        4 => i / 10,
        5 => i,
        _ => panic!("invalid d"),
    }) % 10
}

fn solve_a() -> usize {
    let (amin, amax) = load_input();
    // It is a six-digit number.
    // The value is within the range given in your puzzle input.
    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease;
    // they only ever increase or stay the same (like 111123 or 135679).
    let mut c = 0;
    for i in amin..=amax {
        // TODO: We could probably skip whole ranges on numbers based on these
        // constraints...
        let d = (
            digit(i, 0),
            digit(i, 1),
            digit(i, 2),
            digit(i, 3),
            digit(i, 4),
            digit(i, 5),
        );
        if (d.0 <= d.1 && d.1 <= d.2 && d.2 <= d.3 && d.3 <= d.4 && d.4 <= d.5)
            && (d.0 == d.1 || d.1 == d.2 || d.2 == d.3 || d.3 == d.4 || d.4 == d.5)
        {
            c += 1;
        }
    }
    c
}

fn load_input() -> (usize, usize) {
    let v: Vec<usize> = std::fs::read_to_string("input/input04.txt")
        .unwrap()
        .trim()
        .split('-')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    (v[0], v[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(load_input(), (178416, 676461));
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 1650);
    }
}
