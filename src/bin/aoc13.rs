use std::collections::BTreeMap;

use mbp_aoc2019::intcode::Computer;

pub fn main() {
    println!("13a: {}", solve_a());
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
            map.insert((x, y), tile_id);
        } else {
            panic!("bad chunk? {:?}", chunk);
        }
    }

    map.values().filter(|t| ***t == 2).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 228);
    }
}