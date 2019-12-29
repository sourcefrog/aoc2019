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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Quantity {
    n: usize,
    ident: String,
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Recipe {
    ingredients: Vec<Quantity>,
    product: Quantity,
}

impl Recipe {
    fn product_ident(&self) -> &str {
        &self.product.ident
    }

    fn uses(&self, ingredient_ident: &str) -> bool {
        self.ingredients
            .iter()
            .any(|ing| ing.ident == ingredient_ident)
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

fn parse_ingredients(p: Pair<Rule>) -> Vec<Quantity> {
    let mut v = Vec::new();
    assert_eq!(p.as_rule(), Rule::ingredients);
    // dbg!(&p);
    for i in p.into_inner().flatten() {
        if i.as_rule() == Rule::quantity {
            // dbg!(&i);
            v.push(parse_quantity(i));
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
        let mut ingredients = vec![];
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
    solve_type_a(load())
}

fn solve_type_a(mut rs: Vec<Recipe>) -> usize {
    // needed is the number of units of each type we currently know we need.
    let mut needed = BTreeMap::<String, usize>::new();
    dbg!(&rs);
    needed.insert("FUEL".to_string(), 1);

    loop {
        if needed.len() == 1 && needed.keys().next().unwrap() == "ORE" {
            return *needed.values().next().unwrap();
        }
        // Find a thing we need, that's not itself an ingredient for any remaining recipe.
        let t_ident = needed
            .keys()
            .inspect(|t| println!("check {}", t))
            .find(|t| !rs.iter().any(|r| r.uses(t)))
            .unwrap();
        println!("{} can be removed next", t_ident);
        let t_pos = rs
            .iter()
            .map(Recipe::product_ident)
            .position(|i| i == t_ident)
            .unwrap();
        let t_recipe = rs.swap_remove(t_pos);
        println!("remove recipe {:?}", t_recipe);
        let t_ident = t_recipe.product_ident();

        if let Some(needed_count) = needed.remove(t_ident) {
            let recipe_count = t_recipe.product.n;
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
            for iqty in t_recipe.ingredients.into_iter() {
                *needed.entry(iqty.ident).or_default() += make * iqty.n;
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
}
