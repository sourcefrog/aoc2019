#![allow(dead_code)]

use std::convert::TryInto;

const REPEATS: usize = 10_000;
const PHASES: usize = 100;

pub fn main() {
    //    println!(
    //        "{:?}",
    //    );
    println!("{:?}", solve_a());
    println!("{:?}", solve_b());
}

fn vi8_to_str(a: &[i8]) -> String {
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
    vi8_to_str(
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
    let _a = str_to_vi8(s);
    let _offset: usize = s[..7].parse().unwrap();

    // Calculating everything by brute force over 100M * 100 might be a bit slow...
    //
    // Digit j of the output is the sum of roughly 1/4th of the input digits, minus
    // the sum of 1/4 of the others. These occur at intervals that are predictable
    // from j.
    //
    // It's the sum of blocks of length (j+1), repeating on a cycle of (j+1)*4,
    // starting at position j. And minus same-size blocks half out of phase.
    //
    // So that cuts out a significant bit but we still probably look at all the
    // input...
    //
    // All this is done modulo 10 which seems to perhaps offer some more scope to
    // avoid work...
    //
    // So the FFT is the sum of products of the inputs. If the input is repeated
    // 10k times, that just means the multipliers are larger.
    //
    // I think to calculate output position i (at least in the first cycle)
    // we can find FFT(a)[i] = sum_j(M(i, j) * a[j])
    // and M() can be defined in closed form.
    //
    // For 1 repeat, M is just an expanded cycle of [0, 1, 0, -1] but if the
    // list is to be repeated, then more generally we need to calculate how many
    // times a[j] will occur at positions wtih both additive, and negative,
    // effects, respectively.
    //
    // We could define FFT(a * n)[i] = sum_j(MR(i, j, len(a), r) * a[j]).
    //
    // The counts in MR seem to come from the divmod relationship of len(a), r,
    // j, and i... Which seems a little complicated but probably possible to
    // compute in closed form.
    //
    // But then calculating 100 phases of this seems harder, because there's
    // no guarantee the intermediate input repeats; in fact it seems unlikely
    // to. But, computing a ~6.5M element vector where each element depends on
    // 6.5M inputs is infeasible, and seems like it shouldn't really be
    // necessary. .
    //
    // Could it be possible to compute MRP(i, j, len(a), r, p) which is the
    // multiplier for a[j] to calculate output digit i when a is repeated r
    // times, and the whole thing is run for p phases? It might be possible.
    // It seems like the result would still have this form of a sum of
    // products of the original input, a.
    unimplemented!();
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
}
