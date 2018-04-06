use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate time;

#[derive(Eq, PartialEq, Hash)]
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

fn knapsack2(items: &Vec<Item>, max_weight: usize) -> usize {
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

fn knapsack_inner(items: &Vec<Item>, ref mut memo: &mut HashMap<(usize, i32), usize>, weight: i32, index: usize) -> usize {
    if (index == 0) { return 0 }
    if memo.get(&(index, weight)).is_none() {
        let o1 = knapsack_inner(&items, memo, weight, index - 1);
        let o2 = if weight < items[index -1].weight as i32 { -1000000000} else {knapsack_inner(&items, memo, weight - items[index - 1].weight as i32, index - 1) as i32 };
        memo.insert((index, weight), std::cmp::max(o1 as i32, o2 + items[index - 1].value as i32) as usize);

//         let o1 = knapsack_inner(&items, memo, weight, index - 1);
//         if weight < items[index -1].weight as i32 {
//             memo.insert((index, weight), o1);
//             return o1;
//             } else {
//                 let best = std::cmp::max( o1,
//                     (knapsack_inner(&items, memo, weight - items[index - 1].weight as i32, index - 1) + items[index - 1].value));
//         memo.insert((index, weight), best);
//         return best;
// 
//             };
    }
    *memo.get(&(index, weight)).unwrap()
}

fn knapsack(items: &Vec<Item>, max_weight: usize) -> usize {
    let mut memo: HashMap<(usize, i32), usize> = HashMap::new();

    knapsack_inner(&items, &mut memo, max_weight as i32, items.len());
    *memo.get(&(items.len(), max_weight as i32)).unwrap()
}

fn main() {
    let knapsack_start_time = time::precise_time_ns();
    let (items, max_weight) = load_knapsack("knapsack_big.txt"); // 4243395
    let result = knapsack(&items, max_weight);
    println!("Time to run knapsack: {} ms", //6343
        (time::precise_time_ns() - knapsack_start_time) / 1_000_000);
    println!("knapsack result: {}", result);
    //println!("normal {}", knapsack2(&items, max_weight));
    // assert!(2493893 == knapsack(&items, max_weight))
}


/*


   (4,2), (7,3) (10, 2) (3, 1)




*/
