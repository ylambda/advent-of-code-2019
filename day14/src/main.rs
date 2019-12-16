use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let mut recipes: HashMap<String, Recipe> = HashMap::new();
    let mut inventory: HashMap<String, isize> = HashMap::new();
    inventory.insert("ORE".to_string(), 0);

    for line in input.lines() {
        let recipe = Recipe::from_line(line);
        inventory.insert(recipe.name.to_string(), 0);
        recipes.insert(recipe.name.to_string(), recipe);
    }

    make("FUEL", &mut inventory, &recipes, 2267486);
    let ore = inventory.get("ORE").unwrap();
    println!("Total ORE: {}", ore);

    let max_ores: isize = 1_000_000_000_000;
    if *ore > max_ores {
        println!("Too much: {}", ore);
    }
}

fn make(name: &str, inventory: &mut HashMap<String, isize>, recipes: &HashMap<String, Recipe>, amount: isize) -> isize {
    let recipe = recipes.get(name).unwrap();
    let mut quantity = amount / recipe.amount;
    if quantity <= 0 {
        quantity = 1
    }

    for ingredient in recipe.ingredients.iter() {
        match ingredient.name.as_str() {
            "ORE" => {
                let ore = inventory.get_mut("ORE").unwrap();
                *ore += ingredient.amount * quantity;
            },
            _ => {
                let mut stock = 0;
                let needed = ingredient.amount * quantity;

                {
                    stock = *inventory.get_mut(&ingredient.name).unwrap();
                }

                while stock < needed {
                    let needed = (ingredient.amount * quantity) - stock;
                    let amount = make(&ingredient.name, inventory, recipes, needed);
                    let i_stock = inventory.get_mut(&ingredient.name).unwrap();
                    *i_stock += amount;
                    stock = *i_stock;
                }

                let stock = inventory.get_mut(&ingredient.name).unwrap();
                *stock -= ingredient.amount * quantity;
            }
        }
    }

    recipe.amount * quantity
}

#[derive(Debug)]
struct Recipe {
    name: String,
    amount: isize,
    ingredients: Vec<Ingredient>
}

impl Recipe {

    fn new(name: &str, amount: isize, ingredients: Vec<Ingredient>) -> Recipe {
        Recipe {
            name: name.to_string(),
            amount,
            ingredients
        }
    }

    fn from_line(line: &str) -> Recipe {
        let mut parts = line.split("=>");
        let ingredients: Vec<Ingredient> = parts.next().unwrap()
                .trim()
                .split(",")
                .map(|s| Ingredient::from_str(s.trim()))
                .collect();

        let result = Ingredient::from_str(parts.next().unwrap().trim());

        Self::new(&result.name, result.amount, ingredients)
    }
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    amount: isize,
}

impl Ingredient {

    fn new(name: &str, amount: isize) -> Ingredient {
        Ingredient {
            name: name.to_string(),
            amount
        }
    }

    fn from_str(s: &str) -> Ingredient {
        let mut parts = s.split(" ");
        let amount: isize = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap();

        Self::new(name, amount)
    }

}
