use std::collections::BTreeMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;

pub fn main() {
    println!("14a: {}", solve_a());
}

#[derive(Parser)]
#[grammar = "pest/aoc14.pest"]
struct IdentParser;

type Chemical = String;
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Quantity {
    n: usize,
    ident: Chemical,
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
struct Recipe {
    ingredients: BTreeMap<Chemical, usize>,
    product: Quantity,
}

impl Recipe {
    fn product_ident(&self) -> &str {
        &self.product.ident
    }

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
        ident: ident.unwrap(),
    }
}

fn parse_ingredients(p: Pair<Rule>) -> BTreeMap<Chemical, usize> {
    let mut v = BTreeMap::new();
    assert_eq!(p.as_rule(), Rule::ingredients);
    // dbg!(&p);
    for i in p.into_inner().flatten() {
        if i.as_rule() == Rule::quantity {
            // dbg!(&i);
            let qty = parse_quantity(i);
            let present = v.insert(qty.ident, qty.n).is_some();
            debug_assert!(!present, "chemical occurred twice in ingredient list?");
        }
    }
    // dbg!(&v);
    v
}

fn load() -> Vec<Recipe> {
    parse(&std::fs::read_to_string("input/input14.txt").unwrap())
}

fn parse(s: &str) -> Vec<Recipe> {
    let mut f = IdentParser::parse(Rule::recipe_list, &s).unwrap_or_else(|e| panic!("{}", e));
    let mut recipes = Vec::new();
    for recipe in f.next().unwrap().into_inner() {
        let mut ingredients = BTreeMap::new();
        let mut product = None;
        // println!("Rule:    {:?}", recipe.as_str());
        for ip in recipe.into_inner() {
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

        recipes.push(Recipe {
            ingredients,
            product: product.unwrap(),
        });
    }
    recipes
}

fn solve_a() -> usize {
    solve_type_a(&load())
}

fn solve_type_a(rs: &[Recipe]) -> usize {
    // needed is the number of units of each type we currently know we need.
    let mut needed = BTreeMap::<String, usize>::new();
    dbg!(&rs);
    // Make a copy of the recipes so they can be removed as they're used
    let mut rs: BTreeMap<Chemical, Recipe> = rs
        .iter()
        .cloned()
        .map(|recipe| (recipe.product.ident.clone(), recipe))
        .collect();
    needed.insert("FUEL".to_string(), 1);

    loop {
        if needed.len() == 1 && needed.keys().next().unwrap() == "ORE" {
            return *needed.values().next().unwrap();
        }
        // Find a thing we need, that's not itself an ingredient for any remaining recipe.
        let next_chemical = needed
            .keys()
            .inspect(|t| println!("check {}", t))
            .find(|t| !rs.values().any(|r| r.uses(t)))
            .unwrap()
            .clone();
        println!("{} can be removed next", next_chemical);
        let next_recipe = rs.remove(&next_chemical).unwrap();
        println!("remove recipe {:?}", next_recipe);
        debug_assert_eq!(&next_recipe.product.ident, &next_chemical);
        let t_ident = next_recipe.product_ident();

        if let Some(needed_count) = needed.remove(&next_chemical) {
            let recipe_count = next_recipe.product.n;
            let make = needed_count / recipe_count
                + if (needed_count % recipe_count) > 0 {
                    1
                } else {
                    0
                };
            println!(
                "need {} {}; recipe makes {}; make {}",
                needed_count, t_ident, recipe_count, make
            );
            for (chemical, n) in next_recipe.ingredients.into_iter() {
                *needed.entry(chemical).or_default() += make * n;
            }
            println!("now needed queue is {:?}", needed);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        assert_eq!(load().len(), 60);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            solve_type_a(&parse(
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
}
