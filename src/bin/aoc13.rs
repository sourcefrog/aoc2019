// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate console;

use std::collections::{BTreeMap, BTreeSet};

use console::Term;

use mbp_aoc2019::intcode::Computer;
use mbp_aoc2019::ordering_to_int;

pub fn main() {
    let b = solve_b(true);
    println!("13a: {}", solve_a());
    println!("13b: {}", b);
}

fn solve_a() -> usize {
    let mut g = Game::load();
    g.c.run();
    g.consume_output();
    g.m.values().filter(|t| **t == 2).count()
}

fn solve_b(draw: bool) -> isize {
    // Playing the game successfully actually turns out to be really
    // simple: just keep the paddle under the balls.
    //
    // (I was contemplating something more complicated where we
    // remember previous computer states, essentially as save-points,
    // and rewind if we get something wrong. But it turns out to be
    // unnecessary.)

    let mut g = Game::load();
    let console = &mut Term::stdout();

    g.c.poke_at(0, 2); // Insert a coin :D
    if draw {
        console.clear_screen().unwrap();
    }
    loop {
        g.c.run();
        if g.c.output_len() > 0 {
            g.consume_output();
            if draw {
                console.move_cursor_to(0, 0).unwrap();
                g.draw(console).unwrap();
                // std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        if g.c.wants_input() {
            // Try to keep the paddle under the ball.
            g.c.push_input(ordering_to_int(
                g.ball_pos.unwrap().0.cmp(&g.paddle_pos.unwrap().0),
            ));
        }
        if g.c.is_halted() {
            return g.score;
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
    paddle_pos: Option<Point>,
    ball_pos: Option<Point>,
}

type Point = (isize, isize);

const WALL: isize = 1;
const BLOCK: isize = 2;
const PADDLE: isize = 3;
const BALL: isize = 4;

impl Game {
    fn load() -> Game {
        Game {
            c: Computer::from_file("input/input13.txt"),
            m: BTreeMap::new(),
            score: -1,
            blocks: BTreeSet::new(),
            paddle_pos: None,
            ball_pos: None,
        }
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
                        self.paddle_pos = Some((x, y))
                    } else if t == BALL {
                        self.ball_pos = Some((x, y));
                    }

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
        writeln!(w, "Score: {}", self.score)?;
        let mut s = String::new();
        let ((xmin, xmax), (ymin, ymax)) = self.bounds();
        assert_eq!(ymin, 0);
        assert_eq!(xmin, 0);
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if let Some(t) = self.m.get(&(x, y)) {
                    s.push(match *t {
                        0 => ' ',
                        WALL => '#',
                        BLOCK => '.',
                        PADDLE => '_',
                        BALL => '@',
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 228);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(false), 10776);
    }
}
