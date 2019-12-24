use mbp_aoc2019::{point, Matrix, Point};

const BUG: char = '#';
const EMPTY: char = '.';
const UNKNOWN: char = '?';

type Map = Matrix<char>;

pub fn main() {
    println!("24a: {}", solve_a());
    println!("24b: {}", solve_b());
}

fn solve_a() -> isize {
    solve_type_a(&std::fs::read_to_string("input/input24.txt").unwrap())
}

fn solve_b() -> usize {
    solve_type_b(&std::fs::read_to_string("input/input24.txt").unwrap(), 200)
}

fn solve_type_a(s: &str) -> isize {
    let mut states = vec![Matrix::from_string_lines(s)];
    loop {
        let next = next_state(states.last().unwrap());
        // println!("{}", next.to_string_lines());
        if states.contains(&next) {
            return biodiversity(&next);
        }
        states.push(next);
    }

    // 36742190 is too high?
}

/// Process the recursive grids for the given number of minutes and then return
/// the total number of bugs present.
fn solve_type_b(s: &str, mins: usize) -> usize {
    let mut init = Matrix::from_string_lines(s);
    init[point(2, 2)] = UNKNOWN;
    let mut empty = Matrix::new(5, 5, EMPTY);
    empty[point(2, 2)] = UNKNOWN;
    let empty = empty;
    // stack is always maintained with at least two empty levels on the top
    // and bottom, so we can always look above and below the lowest/highest
    // level that can actually end up with new bugs.
    let mut stack: Vec<Map> = vec![
        empty.clone(),
        empty.clone(),
        init,
        empty.clone(),
        empty.clone(),
    ];

    for _min in 0..mins {
        // println!("start cycle {}, with stack", min);
        // for m in &stack {
        //     println!("{}", m.to_string_lines())
        // }
        // Compute a new stack of levels, one by one, from the bottom up.
        // Every new level can be computed from previous states of this
        // level and the ones above and below it. If there was nothing above
        // or below it, we can just treat them as empty.
        let mut newstack: Vec<Map> = Vec::new();
        newstack.push(empty.clone());
        newstack.push(empty.clone());
        for win in stack.windows(3) {
            let next = stacked_next_state(&win[0], &win[1], &win[2]);
            // println!(
            //     "compute one window:\n{}\n{}\n{}\nresult>>\n{}\n",
            //     &win[0].to_string_lines(),
            //     &win[1].to_string_lines(),
            //     &win[2].to_string_lines(),
            //     &next.to_string_lines()
            // );
            newstack.push(next);
            // println!("{}", newstack.last().unwrap().to_string_lines());
        }
        newstack.push(empty.clone());
        newstack.push(empty.clone());
        stack = newstack;
    }
    stack.iter().map(count_bugs).sum()
}

fn ifbug(m: &Map, p: Point) -> usize {
    (m[p] == BUG) as usize
}

fn stacked_next_state(m0: &Map, m1: &Map, m2: &Map) -> Map {
    let mut nm = Map::new(5, 5, UNKNOWN);
    for p in m1.iter_points() {
        let om = m1[p];
        // Up into the enclosing map
        let m2up = || ifbug(m2, point(2, 1));
        let m2left = || ifbug(m2, point(1, 2));
        let m2right = || ifbug(m2, point(3, 2));
        let m2down = || ifbug(m2, point(2, 3));
        // On the same level
        let m1up = || ifbug(m1, p.up());
        let m1left = || ifbug(m1, p.left());
        let m1right = || ifbug(m1, p.right());
        let m1down = || ifbug(m1, p.down());
        nm[p] = match (p.x, p.y) {
            // Outer corners: look up a level for 2 neighbors
            (0, 0) => stackcell(om, m2up() + m2left() + m1right() + m1down()),
            (4, 0) => stackcell(om, m2up() + m1left() + m2right() + m1down()),
            (0, 4) => stackcell(om, m1up() + m2left() + m1right() + m2down()),
            (4, 4) => stackcell(om, m1up() + m1left() + m2right() + m2down()),
            // Middle of the top row: go up into m2, otherwise stay in this layer.
            (1, 0) | (2, 0) | (3, 0) => stackcell(om, m2up() + m1left() + m1right() + m1down()),
            // Middle of the bottom row
            (1, 4) | (2, 4) | (3, 4) => stackcell(om, m1up() + m1left() + m1right() + m2down()),
            // Middle of the left outside
            (0, 1) | (0, 2) | (0, 3) => stackcell(om, m1up() + m2left() + m1right() + m1down()),
            // Middle of the right outside
            (4, 1) | (4, 2) | (4, 3) => stackcell(om, m1up() + m1left() + m2right() + m1down()),
            // Has 4 neighbors all in the current level
            (1, 1) | (3, 1) | (1, 3) | (3, 3) => {
                stackcell(om, m1up() + m1left() + m1right() + m1down())
            }
            // Directly neighboring the enclosed level, on either side. These have 5 inner neighbors.
            (2, 1) => stackcell(om, m1up() + m1left() + m1right() + sumrowbugs(m0, 0)),
            (2, 3) => stackcell(om, m1down() + m1left() + m1right() + sumrowbugs(m0, 4)),
            (1, 2) => stackcell(om, m1up() + m1left() + m1down() + sumcolbugs(m0, 0)),
            (3, 2) => stackcell(om, m1up() + m1right() + m1down() + sumcolbugs(m0, 4)),
            (2, 2) => UNKNOWN,
            _ => unimplemented!("can't calculate {:?}", p),
        }
    }
    nm
}

fn sumcolbugs(m: &Map, x: isize) -> usize {
    (0..5).map(|y| ifbug(m, point(x, y))).sum()
}

fn sumrowbugs(m: &Map, y: isize) -> usize {
    (0..5).map(|x| ifbug(m, point(x, y))).sum()
}

fn stackcell(s: char, nbugs: usize) -> char {
    if nbugs == 1 || (s == EMPTY && nbugs == 2) {
        BUG
    } else {
        EMPTY
    }
}

fn count_bugs(m: &Map) -> usize {
    m.iter_points().filter(|p| m[*p] == BUG).count()
}

fn next_state(m: &Map) -> Map {
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

fn biodiversity(m: &Map) -> isize {
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

    #[test]
    fn example_b() {
        assert_eq!(solve_type_b(EXAMPLE, 10), 99);
        // solve_type_b(EXAMPLE, 1);
    }
}
