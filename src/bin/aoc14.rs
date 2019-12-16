#![allow(unused_imports)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pest/aoc14.pest"]
struct IdentParser;

#[derive(Debug)]
struct Quantity {
    n: usize,
    thing: String,
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Quantity>,
    result: Quantity,
}

fn parse_quantity(p: Pair<Rule>) -> Quantity {
    assert_eq!(p.as_rule(), Rule::quantity);

    let mut n = None;
    let mut thing = None;
    for i in p.into_inner().flatten() {
        match i.as_rule() {
            Rule::number => n = Some(i.as_str().trim().parse().unwrap()),
            Rule::ident => thing = Some(i.as_str().to_string()),
            _ => (),
        }
    }
    Quantity {
        n: n.unwrap(),
        thing: thing.unwrap(),
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
    dbg!(&v);
    v
}

fn load() -> Vec<Recipe> {
    let s = std::fs::read_to_string("input/input14.txt").unwrap();
    let mut f = IdentParser::parse(Rule::recipe_list, &s).unwrap_or_else(|e| panic!("{}", e));

    let mut recipes = Vec::new();
    for recipe in f.next().unwrap().into_inner() {
        let mut ingredients = vec![];
        let mut result = None;
        println!("Rule:    {:?}", recipe.as_str());
        for ip in recipe.into_inner() {
            match ip.as_rule() {
                Rule::ingredients => {
                    println!("Ingredients: {}", ip.as_str());
                    ingredients = parse_ingredients(ip);
                }
                Rule::result => {
                    result = Some(parse_quantity(ip.into_inner().next().unwrap()));
                }
                _ => println!("Other: {}", ip.as_str()),
            }
        }

        recipes.push(Recipe {
            ingredients,
            result: result.unwrap(),
        });
    }
    recipes
}

pub fn main() {
    dbg!(load());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        assert_eq!(load().len(), 60);
    }
}
