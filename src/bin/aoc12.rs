extern crate num_integer;

use std::cmp::Ordering;
use std::fmt;

use num_integer::Integer;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

pub fn main() {
    dbg!(solve_a());
    dbg!(solve_b());
}

fn solve_a() -> isize {
    let mut ms = make_moons(&[[4, 1, 1], [11, -18, -1], [-2, -10, -4], [-7, -2, 14]]);
    for _ in 0..1000 {
        step(&mut ms);
    }
    total_energy(&ms)
}

fn solve_b() -> usize {
    let mut ms = make_moons(&[[4, 1, 1], [11, -18, -1], [-2, -10, -4], [-7, -2, 14]]);
    let orig_ms = ms.clone();
    // Length of cycles in each of the three dimensions.
    // Cycle occurs when, in dimension d, all moons have velocity 0 and the same
    // position as the initial position.
    let mut cycles = [None; 3];
    for i in 1.. {
        // println!("i={}", i);
        step(&mut ms);
        for (d, cyc) in cycles.iter_mut().enumerate() {
            if cyc.is_none()
                && ms
                    .iter()
                    .enumerate()
                    .all(|(i, m)| m.vel[d] == 0 && m.pos[d] == orig_ms[i].pos[d])
            {
                // println!("Found cycle length {} in d={} at {:?}", i, d, &ms);
                *cyc = Some(i);
            }
        }
        if cycles.iter().all(Option::is_some) {
            break;
        }
    }
    // dbg!(cycles);
    cycles.iter().fold(1usize, |a, b| a.lcm(&b.unwrap()))
}

impl Moon {
    fn new(pos: [isize; 3]) -> Moon {
        Moon {
            pos,
            vel: [0, 0, 0],
        }
    }

    fn energy(&self) -> isize {
        self.pos.iter().cloned().map(isize::abs).sum::<isize>()
            * self.vel.iter().cloned().map(isize::abs).sum::<isize>()
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.pos[0], self.pos[1], self.pos[2], self.vel[0], self.vel[1], self.vel[2]
        )
    }
}

fn ordering_to_int(ord: Ordering) -> isize {
    match ord {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

fn step(ms: &mut [Moon]) {
    // gravity
    // Use indexes to avoid multiply borrowing ms.
    for i in 0..ms.len() {
        for j in 0..ms.len() {
            if i != j {
                let pi = ms[i].pos;
                let mut mj = &mut ms[j];

                mj.vel[0] += ordering_to_int(pi[0].cmp(&mj.pos[0]));
                mj.vel[1] += ordering_to_int(pi[1].cmp(&mj.pos[1]));
                mj.vel[2] += ordering_to_int(pi[2].cmp(&mj.pos[2]));
            }
        }
    }

    // velocity
    for m in ms.iter_mut() {
        m.pos[0] += m.vel[0];
        m.pos[1] += m.vel[1];
        m.pos[2] += m.vel[2];
    }
}

fn make_moons(ms: &[[isize; 3]]) -> Vec<Moon> {
    ms.iter().cloned().map(Moon::new).collect()
}

fn total_energy(ms: &[Moon]) -> isize {
    ms.iter().map(Moon::energy).sum::<isize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn example() {
        let mut ms = make_moons(&[[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]]);

        // after 1 step
        step(&mut ms);
        assert_eq!(ms[0].pos, [2, -1, 1]);
        assert_eq!(ms[0].vel, [3, -1, -1]);

        assert_eq!(ms[1].vel, [1, 3, 3]);
        assert_eq!(ms[1].pos, [3, -7, -4]);

        assert_eq!(ms[2].vel, [-3, 1, -3]);
        assert_eq!(ms[2].pos, [1, -7, 5]);

        assert_eq!(ms[3].pos, [2, 2, 0]);
        assert_eq!(ms[3].vel, [-1, -3, 1]);

        for _ in 2..=10 {
            step(&mut ms);
        }

        assert_eq!(ms.iter().map(Moon::energy).sum::<isize>(), 179);
    }

    #[test]
    fn example_a2() {
        let mut ms = make_moons(&[[-8, -10, 0], [5, 5, 10], [2, -7, 3], [9, -8, -3]]);
        for _i in 1..=100 {
            step(&mut ms);
        }
        assert_eq!(total_energy(&ms), 1940);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 9493);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 326_365_108_375_488);
    }
}
