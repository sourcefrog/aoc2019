/// Generate all permutations of the elements in 0..n.
pub fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec![vec![0]]
    } else {
        // The number n-1 inserted at every possible position, in every possible
        // permutation of the other numbers.
        let m = n - 1;
        let mut r = Vec::new();
        for p in permutations(m) {
            for i in 0..=(p.len()) {
                let mut q = p.clone();
                q.insert(i, m);
                r.push(q);
            }
        }
        r
    }
}

#[cfg(test)]
mod test {
    fn pts(n: usize) -> String {
        super::permutations(n)
            .iter()
            .map(|e| {
                e.iter()
                    .map(usize::to_string)
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn permute() {
        assert_eq!(pts(0), "");
        assert_eq!(pts(1), "0");
        assert_eq!(pts(2), "10,01");
        assert_eq!(pts(3), "210,120,102,201,021,012");

        assert_eq!(super::permutations(6).len(), 6 * 5 * 4 * 3 * 2);
    }
}
