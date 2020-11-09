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

use std::collections::BTreeMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;

const TRILLION: u64 = 1_000_000_000_000;

pub fn main() {
    println!("14a: {}", solve_a());
    println!("14b: {}", solve_b());
}

fn solve_a() -> u64 {
    solve_type_a(load())
}

fn solve_b() -> u64 {
    solve_type_b(load())
}

#[derive(Parser)]
#[grammar = "pest/aoc14.pest"]
struct IdentParser;

type Chemical = String;
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Quantity {
    n: u64,
    chemical: Chemical,
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
struct Recipe {
    ingredients: BTreeMap<Chemical, u64>,
    product: Quantity,
}

/// Map from chemical produced to recipe.
type RecipeMap = BTreeMap<Chemical, Recipe>;

impl Recipe {
    fn uses(&self, chemical: &str) -> bool {
        self.ingredients.contains_key(chemical)
    }
}

fn parse_quantity(p: Pair<Rule>) -> Quantity {
    assert_eq!(p.as_rule(), Rule::quantity);

    let mut n = None;
    let mut ident = None;
    for i in p.into_inner().flatten() {
        match i.as_rule() {
            Rule::number => n = Some(i.as_str().parse().unwrap()),
            Rule::ident => ident = Some(i.as_str().to_string()),
            _ => (),
        }
    }
    Quantity {
        n: n.unwrap(),
        chemical: ident.unwrap(),
    }
}

fn parse_ingredients(p: Pair<Rule>) -> BTreeMap<Chemical, u64> {
    let mut v = BTreeMap::new();
    assert_eq!(p.as_rule(), Rule::ingredients);
    // dbg!(&p);
    for i in p.into_inner().flatten() {
        if i.as_rule() == Rule::quantity {
            // dbg!(&i);
            let qty = parse_quantity(i);
            let present = v.insert(qty.chemical, qty.n).is_some();
            debug_assert!(!present, "chemical occurred twice in ingredient list?");
        }
    }
    // dbg!(&v);
    v
}

fn load() -> RecipeMap {
    parse(&std::fs::read_to_string("input/input14.txt").unwrap())
}

fn parse(s: &str) -> RecipeMap {
    let mut f = IdentParser::parse(Rule::recipe_list, &s).unwrap_or_else(|e| panic!("{}", e));
    let mut recipes = BTreeMap::new();
    for recipe_span in f.next().unwrap().into_inner() {
        let mut ingredients = BTreeMap::new();
        let mut product = None;
        // println!("Rule:    {:?}", recipe.as_str());
        for ip in recipe_span.into_inner() {
            match ip.as_rule() {
                Rule::ingredients => {
                    // println!("Ingredients: {}", ip.as_str());
                    ingredients = parse_ingredients(ip);
                }
                Rule::product => {
                    product = Some(parse_quantity(ip.into_inner().next().unwrap()));
                }
                _ => println!("Other: {}", ip.as_str()),
            }
        }
        let product = product.expect("No product found");
        let already_present = recipes
            .insert(
                product.chemical.clone(),
                Recipe {
                    product,
                    ingredients,
                },
            )
            .is_some();
        debug_assert!(!already_present);
    }
    recipes
}

fn solve_type_a(rs: RecipeMap) -> u64 {
    make_n_fuel(rs, 1)
}

/// Find by bisection the amount of fuel that can be produced by a TRILLION ORE.
fn solve_type_b(rm: RecipeMap) -> u64 {
    // An amount of fuel that we know can be produced.
    let mut low_bound: u64 = 1;
    // An amount of fuel that's more than we can produce.
    let mut high_bound: u64 = TRILLION;
    while low_bound + 1 != high_bound {
        let guess = (low_bound + high_bound) / 2;
        if make_n_fuel(rm.clone(), guess) > TRILLION {
            high_bound = guess
        } else {
            low_bound = guess
        }
    }
    low_bound
}

/// True if any recipe in this map uses this chemical.
fn any_recipe_uses(rm: &RecipeMap, chemical: &Chemical) -> bool {
    rm.values().any(|r| r.uses(chemical))
}

/// Returns the amount of ORE required to make n_fuel FUEL.
fn make_n_fuel(mut rs: RecipeMap, n_fuel: u64) -> u64 {
    // `needed` is the number of units of each type we currently know we need.
    let mut needed = BTreeMap::<Chemical, u64>::new();
    needed.insert("FUEL".to_string(), n_fuel);

    loop {
        if needed.len() == 1 {
            if let Some(needed_ore) = needed.get("ORE") {
                return *needed_ore;
            }
        }
        // Find a thing we need, that's not itself an ingredient for any remaining recipe.
        let next_chemical = needed
            .keys()
            .find(|chemical| !any_recipe_uses(&rs, chemical))
            .unwrap()
            .clone();
        let next_recipe = rs.remove(&next_chemical).unwrap();
        debug_assert_eq!(&next_recipe.product.chemical, &next_chemical);

        // If none of this chemical is needed, that's ok, we can still remove it from
        // the active list. Otherwise, calculate the right amount to make.
        if let Some(needed_count) = needed.remove(&next_chemical) {
            let recipe_count = next_recipe.product.n;
            let make = (needed_count + recipe_count - 1) / recipe_count;
            for (chemical, n) in next_recipe.ingredients.into_iter() {
                *needed.entry(chemical).or_default() += make * n;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RECIPE_13312: &str = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const RECIPE_180697: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";

    const RECIPE_2210736: &str = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX
    ";

    #[test]
    fn parse_input() {
        assert_eq!(load().len(), 60);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            solve_type_a(parse(
                "\
                    10 ORE => 10 A
                    1 ORE => 1 B
                    7 A, 1 B => 1 C
                    7 A, 1 C => 1 D
                    7 A, 1 D => 1 E
                    7 A, 1 E => 1 FUEL
                    ",
            )),
            31
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 178_154)
    }

    #[test]
    fn example_b_1() {
        let ore_required = make_n_fuel(parse(RECIPE_13312), 82892753);
        dbg!(ore_required);
        assert!(ore_required < TRILLION);
    }

    #[test]
    fn example_b_2() {
        let ore_required = make_n_fuel(parse(RECIPE_180697), 5586022);
        dbg!(ore_required);
        assert!(ore_required < TRILLION);
    }
    #[test]
    fn example_b_3() {
        let ore_required = make_n_fuel(parse(RECIPE_2210736), 460664);
        dbg!(ore_required);
        assert!(ore_required < TRILLION);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 6226152);
    }
}
