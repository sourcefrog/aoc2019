//! Find the shortest path in a graph, using Djikstra's method.

use std::collections::{BTreeMap, BinaryHeap};

type D = isize;

/// Find the shortest path in a graph, using Djikstra's method.
///
/// Positions are identified by type `P` which might be a `Point` or something
/// more complicated to describe additional state. Distances are measured
/// as isizes.
///
/// This takes a callback which returns all the neighbors from `p: P` and
/// the incremental distance to them, as tuples. The neighbor callback is mut to allow
/// for internal caching.
pub fn shortest_distance<P, N>(origin: P, dest: P, nbr_fn: &mut N) -> D
where
    P: Eq + Ord + Copy,
    N: FnMut(P) -> Vec<(P, D)>,
{
    shortest_distance_fn(origin, |&p| dest == p, nbr_fn)
}

/// Calculate the shortest distance, with a callback that says whether a point is the destination.
pub fn shortest_distance_fn<P, N, DF>(origin: P, dest_fn: DF, nbr_fn: &mut N) -> D
where
    P: Eq + Ord + Copy,
    N: FnMut(P) -> Vec<(P, D)>,
    DF: Fn(&P) -> bool,
{
    // Next points to visit, indexed by the *negative* distance, so that the
    // greatest value is the shortest.
    let mut queue = BinaryHeap::<(D, P)>::new();
    // Shortest known distance to reach any point.
    let mut best = BTreeMap::<P, D>::new();
    queue.push((0, origin));
    loop {
        let (d, p) = queue
            .pop()
            .expect("heap is empty without reaching destination");
        let d = -d;
        if dest_fn(&p) {
            // Found a shortest path to the end
            return d;
        }
        for (np, step) in nbr_fn(p) {
            let nd = step + d;
            if let Some(prev_d) = best.get(&np) {
                if nd >= *prev_d {
                    continue; // Already found a shorter path; don't revisit.
                }
            }
            best.insert(np, nd);
            queue.push((-nd, np));
        }
    }
}
