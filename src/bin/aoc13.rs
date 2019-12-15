use std::collections::{BTreeMap, BTreeSet};

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("13a: {}", solve_a());
    println!("13b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut g = Game::load();
    g.dry_run();
    g.m.values().filter(|t| **t == 2).count()
}

fn solve_b() -> isize {
    Game::load().play()
}

struct Game {
    c: Computer,
    m: BTreeMap<(isize, isize), isize>,
    score: isize,

    // Positions of remaining blocks
    blocks: BTreeSet<Point>,
    // Position of paddle
    paddle: Option<Point>,
}

type Point = (isize, isize);

const BLOCK: isize = 2;
const PADDLE: isize = 3;

impl Game {
    fn load() -> Game {
        Game {
            c: Computer::from_file("input/input13.txt"),
            m: BTreeMap::new(),
            score: -1,
            blocks: BTreeSet::new(),
            paddle: None,
        }
    }

    fn dry_run(&mut self) {
        self.c.run();
        self.consume_output();
        println!("{}", self.draw());
    }

    fn consume_output(&mut self) {
        for chunk in self.c.drain_output().chunks(3) {
            if let [x, y, t] = chunk {
                let x = *x;
                let y = *y;
                let t = *t;
                // dbg!(&x, &y, &tile_id);
                if x == -1 && y == 0 {
                    self.score = t;
                } else {
                    self.m.insert((x, y), t);
                    if t == PADDLE {
                        self.paddle = Some((x, y))
                    };
                    if t == BLOCK {
                        self.blocks.insert((x, y));
                    } else {
                        self.blocks.remove(&(x, y));
                    }
                }
            } else {
                panic!("bad chunk? {:?}", chunk);
            }
        }
    }

    fn draw(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Score: {}\n", self.score));
        let ((xmin, xmax), (ymin, ymax)) = self.bounds();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if let Some(t) = self.m.get(&(x, y)) {
                    s.push(match t {
                        0 => ' ',
                        1 => '#',
                        2 => '.',
                        3 => '_',
                        4 => '0',
                        _ => panic!(),
                    });
                }
            }
            s.push('\n');
        }
        s
    }

    fn bounds(&self) -> ((isize, isize), (isize, isize)) {
        (
            (
                self.m.keys().map(|i| i.0).min().unwrap(),
                self.m.keys().map(|i| i.0).max().unwrap(),
            ),
            (
                self.m.keys().map(|i| i.1).min().unwrap(),
                self.m.keys().map(|i| i.1).max().unwrap(),
            ),
        )
    }

    // To play the game: the only thing we control is the position of the
    // paddle, and it seems like all we can do is put that in the position
    // the ball will next come down to the bottom line.
    //
    // The field is wide enough that it looks like we can't just wait until
    // the ball is moving down and then move the paddle, because it's much
    // wider than it is deep...
    //
    // Although actually, rather than simulating the game, I wonder if
    // we can just peek and work out what value will be output?
    //
    // The (probably too obvious) guess would be it's one point per block...
    // But it's not.
    //
    // Q: Do we need to predict where the ball will bounce, projecting it through
    // all collisions? I wonder how predictable it is...
    //
    // Q: It seems we don't know which direction the ball is initally travelling.
    //
    // This seems to also actually require teaching the computer to run until it
    // wants input and then stop.
    fn play(&mut self) -> isize {
        self.c.poke_at(0, 2);
        while !self.blocks.is_empty() {}
        self.score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 228);
    }
}
