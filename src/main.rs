use std::cmp::Ordering;
use std::collections::BinaryHeap;

use rand::seq::SliceRandom;

#[derive(Eq, PartialEq, Debug)]
enum Fruit {
    Fig,
    Other(String),
}

// We define fig as the highest priority by implementing ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = rand::thread_rng();
    let fruits = vec![
        "Apple",
        "Pear",
        "Cherry",
        "Ferry",
        "Lemon",
        "Strawberry",
        "Peach",
        "Pear",
        "Orange",
        "Pineapple",
        "Lemon",
        "Fig",
        "Fig",
        "Fig",
        "Fig",
    ];

    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn main() {
    let fruit_salad = generate_fruit_salad();
    println!("{:?}", fruit_salad);
}
