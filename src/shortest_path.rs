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

//! Find the shortest path in a graph, using Djikstra's method.

use core::hash::Hash;
use std::collections::{HashMap, BinaryHeap};

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
    P: Eq + Ord + Copy + Hash,
    N: FnMut(P) -> Vec<(P, D)>,
{
    shortest_distance_fn(origin, |&p| dest == p, nbr_fn)
}

/// Calculate the shortest distance, with a callback that says whether a point is the destination.
pub fn shortest_distance_fn<P, N, DF>(origin: P, dest_fn: DF, nbr_fn: &mut N) -> D
where
    P: Eq + Ord + Copy + Hash,
    N: FnMut(P) -> Vec<(P, D)>,
    DF: Fn(&P) -> bool,
{
    // Next points to visit, indexed by the *negative* distance, so that the
    // greatest value is the shortest.
    let mut queue = BinaryHeap::<(D, P)>::new();
    // Shortest known distance to reach any point.
    let mut best = HashMap::<P, D>::new();
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
