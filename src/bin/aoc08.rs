extern crate bytecount;

use std::convert::TryInto;

pub fn main() {
    println!("08a: {}", solve_a());
}

fn solve_a() -> usize {
    let im = Image::from_string(&load_input(), 25, 6);
    // Find the layer with the fewest zeros.
    let least_zeros_layer = (0..(im.l))
        .map(|i| (im.count_in_layer(0, i), i))
        .min()
        .unwrap()
        .1;
    im.count_in_layer(1, least_zeros_layer) * im.count_in_layer(2, least_zeros_layer)
}

struct Image {
    digits: Vec<u8>,
    w: usize,
    h: usize,
    /// Number of layers
    l: usize,
}

impl Image {
    pub fn from_string(s: &str, w: usize, h: usize) -> Image {
        let s = s.trim();
        // Check it's an even number of planes
        assert_eq!(s.len() % (w * h), 0);
        Image {
            digits: s
                .chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect(),
            w,
            h,
            l: s.len() / (w * h),
        }
    }

    /// Return a reference to a slice of the image in the given layer.
    pub fn layer(&self, p: usize) -> &[u8] {
        assert!(p < self.l);
        let sz = self.w * self.h;
        let a = sz * p;
        &self.digits[a..(a + sz)]
    }

    /// Return the count of digits equal to `d` within layer `l`.
    pub fn count_in_layer(&self, d: u8, l: usize) -> usize {
        bytecount::count(self.layer(l), d)
    }
}

fn load_input() -> String {
    std::fs::read_to_string("input/input08.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 2032);
    }
}
