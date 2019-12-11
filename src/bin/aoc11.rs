use std::collections::BTreeMap;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("{}", solve_a());
}

fn solve_a() -> usize {
    // contains squares that have been painted, and their color.
    // false or unset is black; true is white.
    let mut painted = BTreeMap::<(isize, isize), bool>::new();
    let mut pos = (0, 0);
    // 0=up, 1=left, etc.
    let mut dir = 0;
    let mut c = Computer::from_file("input/input11.txt");
    loop {
        c.clear_input(); // in case it wasn't read
        c.push_input(painted.get(&pos).cloned().unwrap_or_default().into());
        if let Some(paint_this) = c.run_until_output() {
            println!("paint {}", paint_this);
            assert!(paint_this == 0 || paint_this == 1);
            painted.insert(pos, paint_this != 0);
        } else {
            break;
        }
        if let Some(turn) = c.run_until_output() {
            println!("turn {}", turn);
            dir = match turn {
                0 => (dir + 3) % 4,
                1 => (dir + 1) % 4,
                _ => panic!(),
            };
            pos = match dir {
                0 => (pos.0, pos.1 - 1),
                1 => (pos.0 + 1, pos.1),
                2 => (pos.0, pos.1 + 1),
                3 => (pos.0 - 1, pos.1),
                _ => panic!("bad dir {}", dir),
            };
        } else {
            break;
        }
    }
    // The number of unique squares painted, even if repeatedly.
    painted.len()
}
