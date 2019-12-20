#![allow(dead_code)]

use std::convert::TryInto;

const REPEATS: usize = 10_000;
const PHASES: usize = 100;

pub fn main() {
    //    println!(
    //        "{:?}",
    //    );
    //    println!("{:?}", solve_a());
    // println!("{:?}", solve_b());

    let s = "03036732577212944063491565474664";
    let a = str_to_vi8(s);
    println!("direct fftn:\n{}", vi8_to_string(&fftn(a, 100)));

    let mut ss = s.to_string();
    ss.push_str(s);
    println!("doubled input:\n{}", &ss);
    println!(
        "doubled fftn:\n{}",
        vi8_to_string(&fftn(str_to_vi8(&ss), 100))
    );
}

fn vi8_to_string(a: &[i8]) -> String {
    let mut s = String::new();
    for i in a {
        s.push_str(&i.to_string());
    }
    s
}

fn str_to_vi8(s: &str) -> Vec<i8> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}

fn fft1(a: &[i8]) -> Vec<i8> {
    let mut o = Vec::with_capacity(a.len());
    for (i, _x) in a.iter().enumerate() {
        let mut s: isize = 0;
        // Take blocks of length (i+1) every (i+1)*4, starting at i.
        for (j, y) in a.iter().enumerate() {
            let phase = ((j + 1) / (i + 1)) % 4;
            let mul = [0, 1, 0, -1][phase];
            // print!("{}*{} ", y, mul,);
            s += mul * (*y as isize);
        }
        let s = s.abs() % 10;
        // println!("= {}", s);
        o.push(s.try_into().unwrap());
    }
    o
}

fn solve_a() -> String {
    vi8_to_string(
        &fftn(
            str_to_vi8(&std::fs::read_to_string("input/input16.txt").unwrap()),
            100,
        )[..8],
    )
}

fn solve_b() -> String {
    let instr = &std::fs::read_to_string("input/input16.txt").unwrap();
    solve_type_b(&instr)
}

fn solve_type_b(s: &str) -> String {
    let s = s.trim();
    let _offset: usize = s[..7].parse().unwrap();

    let a = fftn(str_to_vi8(s), 100);
    vi8_to_string(&a)
    // let offmod = offset % s.len();
    // vi8_to_string(&a[offmod..offmod + 7])
}

fn fftn(mut a: Vec<i8>, n: usize) -> Vec<i8> {
    for _ in 0..n {
        a = fft1(&a);
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        // fft1(&str_to_vi8("12345678"));
        assert_eq!(fft1(&str_to_vi8("12345678")), str_to_vi8("48226158"));

        assert_eq!(fftn(str_to_vi8("12345678"), 4), str_to_vi8("01029498"));
    }

    #[test]
    fn example_b() {
        // 03036732577212944063491565474664 becomes 84462026.
        // 02935109699940807407585447034323 becomes 78725270.
        // 03081770884921959731165446850517 becomes 53553731.
        assert_eq!(
            solve_type_b("03036732577212944063491565474664",),
            "84462026"
        );
    }
}
