use std::{collections::hash_map::Entry, fmt::Display};

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct Part2 {
    ingredients: Vec<(&'static str, &'static str)>,
}

impl Display for Part2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.ingredients.iter();
        write!(f, "{}", it.next().unwrap().0)?;
        it.try_for_each(|(ingredient, _)| write!(f, ",{}", ingredient))
    }
}

#[inline]
pub fn solve() -> (usize, Part2) {
    let mut allergen_possibilities: HashMap<&str, HashSet<&str>> = HashMap::default();
    let mut all_ingredients: HashMap<&str, usize> = HashMap::default();

    include_str!("input.txt").lines().for_each(|food| {
        if food.is_empty() {
            return;
        }

        let (ingredients, allergens) = {
            let mut it = food.splitn(2, '(');
            (it.next().unwrap(), it.next().unwrap())
        };

        let ingredients: HashSet<&str> = ingredients.trim().split(' ').collect();
        let allergens = allergens["contains ".len()..allergens.len() - 1].split(", ");

        for allergen in allergens {
            match allergen_possibilities.entry(allergen) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().retain(|foo| ingredients.contains(foo))
                }

                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            }
        }

        for ingredient in ingredients {
            *all_ingredients.entry(ingredient).or_default() += 1;
        }
    });

    println!("{}", all_ingredients.len());

    for possibilities in allergen_possibilities.values() {
        all_ingredients.retain(|ingredient, _| !possibilities.contains(ingredient));
    }

    let part1 = all_ingredients.values().sum();

    let mut ingredients = Vec::new();

    while let Some((&allergen, ingredient)) =
        allergen_possibilities.iter().find(|(_, vs)| vs.len() == 1)
    {
        let ingredient = *ingredient.into_iter().next().unwrap();

        ingredients.push((ingredient, allergen));

        allergen_possibilities.remove(allergen);
        allergen_possibilities
            .iter_mut()
            .for_each(|(_, possibilities)| {
                possibilities.remove(ingredient);
            });
    }

    ingredients.sort_unstable_by_key(|(_, allergen)| *allergen);

    (part1, Part2 { ingredients })
}
