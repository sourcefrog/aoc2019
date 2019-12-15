#![allow(dead_code)]

// Here's my approach to part B, working out the final score.
//
// The only thing we control is the position of the paddle, by giving
// incremental {-1, 0, +1} inputs to move it each time the game stops
// to wait for input.
//
// I assume the game is fairly deterministic (there is no randomness source)
// and doesn't change in response to paddle input other than moving the paddle
// (and therefore where the ball bounces.)o
//
// We can tell what the score is, where the ball is, and where the paddle
// is. We can tell whether we lost because the ball will go below
// the paddle. (And perhaps the game will stop? Or perhaps it lets you
// keep futilely moving the paddle?)
//
// We can tell if we won because there's no blocks left, and, again, perhaps
// the game will stop.
//
// So the approach is: remember all previous game states, giving effectively
// a save-point at every input. Try to play the game.
// If we miss the ball with the paddle, rewind to that state, remember the
// goal paddle location, and rewind however many input steps are necessary
// to get to that position in time.  Keep trying this until there are no
// blocks left, and we've won.
//
// This avoids needing to predict where the ball will bounce.
//
// An alternative approach, and perhaps simpler, approach, would be to keep
// the paddle always underneath the ball. But I'm concerned that we would
// actually need to lead the ball with the paddle (if it moves diagonally
// as it reaches the paddle) and that might be complicated, or we might be
// left behind. But, perhaps it's simpler to start with?

extern crate console;

use std::collections::{BTreeMap, BTreeSet};

use console::Term;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    // println!("13a: {}", solve_a());
    println!("13b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut g = Game::load();
    g.dry_run();
    g.m.values().filter(|t| **t == 2).count()
}

fn solve_b() -> isize {
    let mut g = Game::load();

    let mut console = Term::stdout();

    // Initial dry run to draw the screen
    // g.dry_run();

    // Insert a coin :D
    g.c.poke_at(0, 2);
    // Run - will wait for input.
    loop {
        g.c.run();
        if g.c.wants_input() {
            println!("push 0");
            g.c.push_input(0);
        } 
        if g.c.output_len() > 0 {
            g.consume_output();
            console.clear_screen().unwrap();
            g.draw(&mut console).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        } 
        if g.c.is_halted() {
            println!("halted: score {}", g.score);
            return g.score
        }
    }
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
        self.draw(&mut std::io::stdout()).unwrap();
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

    fn draw(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "Score: {}\n", self.score)?;
        let mut s = String::new();
        let ((xmin, xmax), (ymin, ymax)) = self.bounds();
        assert_eq!(ymin, 0);
        assert_eq!(xmin, 0);
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if let Some(t) = self.m.get(&(x, y)) {
                    s.push(match t {
                        0 => ' ',
                        1 => '#',
                        2 => '.',
                        3 => '_',
                        4 => '@',
                        _ => panic!(),
                    });
                }
            }
            s.push('\n');
        }
        w.write_all(s.as_bytes())
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
