use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Item {
    value: usize,
    weight: usize,
}

impl Item {
    fn from_str(line: &str) -> Item {
        let parts = line.split(" ").collect::<Vec<&str>>();
        return Item {
            value: parts[0].parse::<usize>().unwrap(),
            weight: parts[1].parse::<usize>().unwrap()
        }
    }
}

fn load_knapsack(filename: &str) -> (Vec<Item>, usize) {
    let f = File::open(filename).expect("Error opening file");
    let mut reader = BufReader::new(f);
    let mut top_line = String::new();
    reader.read_line(&mut top_line).unwrap();
    let max_weight = top_line.split(" ").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();

    let items = reader.lines()
        .map(|line| Item::from_str(&line.unwrap()))
        .collect::<Vec<Item>>();

    (items, max_weight)
}

fn knapsack(items: &Vec<Item>, max_weight: usize) -> usize {
    let mut memo = vec![vec![0; max_weight + 1]; items.len() + 1];
    for i in 0..items.len() {
        for j in 1..max_weight + 1 {
            memo[i + 1][j] =
                if j >= items[i].weight && memo[i][j] < memo[i][j - items[i].weight] + items[i].value {
                    memo[i][j - items[i].weight] + items[i].value
                } else {
                    memo[i][j]
                }
        }
    }
    memo[items.len()][max_weight]
}

fn main() {
    let (items, max_weight) = load_knapsack("knapsack1.txt");
    println!("{}", knapsack(&items, max_weight));
    // assert!(2493893 == knapsack(&items, max_weight))
}
