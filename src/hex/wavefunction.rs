use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bitvec::prelude::*;
use multimap::MultiMap;
use rand::prelude::*;

use super::{Cube, CUBE_DIRECTIONS};

#[derive(Clone)]
struct Tile {
    allowed: BitVec,
}

impl Tile {
    pub fn new(num_states: usize) -> Self {
        Self {
            allowed: BitVec::repeat(true, num_states),
        }
    }
    pub fn apply_restrictions(&mut self, restriction: &BitVec) -> (bool, bool) {
        let old_ones = self.allowed.count_ones();
        self.allowed &= restriction;

        let ones = self.allowed.count_ones();
        (ones == 1, ones != old_ones)
    }

    pub fn collapse(&mut self, weights: &[f32]) {
        let mut rng = rand::thread_rng();
        let candidates = self.allowed.iter_ones().collect::<Vec<_>>();
        let actual = candidates
            .choose_weighted(&mut rng, |i| weights[*i])
            .unwrap();
        // let actual = self.allowed.iter_ones().choose(&mut rng, |c| {}).unwrap();
        self.allowed.fill(false);
        self.allowed.set(*actual, true);
    }
}

pub fn test(input_tiles: &HashMap<Cube, usize>) -> impl Iterator<Item = (Cube, usize)> {
    let weights = vec![0.50, 0.05, 0.40, 0.05];

    let rules = [
        (0, 1),
        (1, 0),
        (1, 2),
        (2, 1),
        (2, 3),
        (3, 2),
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
    ]
    .iter()
    .cloned()
    .collect::<MultiMap<usize, usize>>();

    let mut tiles: HashMap<Cube, Tile> = HashMap::new();
    let mut uncollapsed = HashSet::new();

    {
        let mut dirty = Vec::new();

        for y in 0..13 {
            for x in 0..20 {
                let v = Vec2::new(x as f32, y as f32);
                let k = Cube::from_odd_r(v);
                let mut tile = Tile::new(4);

                if let Some(x) = input_tiles.get(&k) {
                    tile.allowed.fill(false);
                    tile.allowed.set(*x, true);

                    dirty.push(k);
                } else {
                    uncollapsed.insert(k);
                }
                tiles.insert(k, tile);
            }
        }
        // for y in 0..13 {
        //     for x in 0..20 {
        //         let v = Vec2::new(x as f32, y as f32);
        //         let k = Cube::from_odd_r(v);
        //         if y == 0 || y == 12 || (x == 0 || x == 19) {
        //             let tile = tiles.get_mut(&k).unwrap();
        //             tile.allowed.fill(false);
        //             tile.allowed.set(0, true);
        //             dirty.push(k);
        //         }
        //     }
        // }

        while let Some(d) = dirty.pop() {
            let dirty_tile = tiles.get(&d).unwrap();
            let allowed_states = dirty_tile.allowed.clone();
            for ndir in CUBE_DIRECTIONS.iter() {
                let n = d + *ndir;
                if let Some(neighbor_tile) = tiles.get_mut(&n) {
                    let restrict = derive_neighbor_restriction(&allowed_states, &rules);
                    let (collapsed, changed) = neighbor_tile.apply_restrictions(&restrict);
                    if collapsed {
                        uncollapsed.remove(&n);
                    }
                    if changed {
                        dirty.push(n);
                    }
                }
            }
            // println!("dirty: {:?}", dirty);
        }
    }
    let mut rng = rand::thread_rng();
    // uncollapsed = uncollapsed.difference(&remove_tiles);
    // let mut step_mode = true;
    while !uncollapsed.is_empty() {
        // let collapse_i = rng.gen_range(0..unstable.len());
        let collapse = *uncollapsed.iter().choose(&mut rng).unwrap();
        uncollapsed.remove(&collapse);

        let tile = tiles.get_mut(&collapse).unwrap();

        debug!("allowed: {:?}", tile.allowed);
        tile.collapse(&weights);

        let mut dirty = vec![collapse];
        while let Some(d) = dirty.pop() {
            let dirty_tile = tiles.get(&d).unwrap();
            let allowed_states = dirty_tile.allowed.clone();
            // let d: Cube = d;
            for ndir in CUBE_DIRECTIONS.iter() {
                let n = d + *ndir;
                if let Some(neighbor_tile) = tiles.get_mut(&n) {
                    let restrict = derive_neighbor_restriction(&allowed_states, &rules);
                    let (collapsed, changed) = neighbor_tile.apply_restrictions(&restrict);
                    if collapsed {
                        uncollapsed.remove(&n);
                    }
                    if changed {
                        dirty.push(n);
                    }
                }
            }
            // println!("dirty: {:?}", dirty);
        }
    }
    tiles
        .into_iter()
        .map(|(p, t)| (p, t.allowed.first_one().unwrap()))
}

fn derive_neighbor_restriction(new_restriction: &BitVec, rules: &MultiMap<usize, usize>) -> BitVec {
    let mut restrict = BitVec::repeat(false, new_restriction.len());
    for a in new_restriction.iter_ones() {
        if let Some(bv) = rules.get_vec(&a) {
            for b in bv {
                restrict.set(*b, true)
            }
        }
    }
    restrict
}
