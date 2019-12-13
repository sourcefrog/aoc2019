use std::collections::BTreeMap;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("13a: {}", solve_a());
    println!("13b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut c = Computer::from_file("input/input13.txt");
    c.run();
    let o = c.drain_output();
    assert!(o.len() % 3 == 0);

    let mut map = BTreeMap::new();
    for chunk in o.chunks(3) {
        if let [x, y, tile_id] = chunk {
            // dbg!(&x, &y, &tile_id);
            map.insert((*x, *y), *tile_id);
        } else {
            panic!("bad chunk? {:?}", chunk);
        }
    }

    let map = Map { m: map };
    println!("{}", map.draw());

    map.m.values().filter(|t| **t == 2).count()
}

fn solve_b() -> usize {
    let mut c = Computer::from_file("input/input13.txt");
    0
}

struct Map {
    m: BTreeMap<(isize, isize), isize>,
}

impl Map {
    fn draw(&self) -> String {
        let mut s = String::new();
        let ((xmin,xmax),(ymin,ymax)) = self.bounds();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if let Some(t) = self.m.get(&(x, y)) {
                    s.push(match t {
                        0 => ' ',
                        1 => '#',
                        2 => 'a',
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 228);
    }
}
