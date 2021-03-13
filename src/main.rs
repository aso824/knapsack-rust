#[macro_use] extern crate serde_derive;
extern crate serde;

mod goal;
mod item;

use item::Items;
use goal::goal;

use std::env;
use std::fs::File;
use rand::{thread_rng};
use rand::seq::SliceRandom;
use crate::item::Item;

fn random_with_limit(items: &Items, limit: f32) -> Items {
    let mut available: Vec<Item> = items.0.to_vec();
    available.shuffle(&mut thread_rng());

    let mut result = Items(vec![]);
    let mut weight_sum: f32 = 0.0;

    loop {
        let new_item = match available.pop() {
            Some(item) => item,
            None => break
        };

        if new_item.weight > limit - weight_sum {
            continue;
        }

        weight_sum += new_item.weight;
        result.0.push(new_item);

        if weight_sum == limit || available.is_empty() {
            break;
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: ./knapsack [input.json] [maximum weight]");

        std::process::exit(1);
    }

    let all_items = match File::open(&args[1]) {
        Ok(file) => {
            match Items::load(file) {
                Some(items) => items,
                None => {
                    eprintln!("Failed to load input file - check if structure is valid");

                    std::process::exit(3);
                }
            }
        },
        Err(_) => {
            eprintln!("Failed to open input file");

            std::process::exit(2);
        }
    };

    println!("{:#?}", all_items);
    println!("Value sum: {:.2}", crate::goal(&all_items));

    let knapsack = random_with_limit(&all_items, args[2].parse::<f32>().unwrap());

    println!("{:#?}", knapsack);
    println!("Value sum: {:.2}", crate::goal(&knapsack));
}
