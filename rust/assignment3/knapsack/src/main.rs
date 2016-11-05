use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

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
//     for i in &memo {
//         for j in i {
//             print!("{} ", j);
//         }
//         println!("");
//     }
    memo[items.len()][max_weight]
}

fn printMemo(memo: &HashMap<(usize, i32), usize>) {
    for i in memo {
        println!("{} {} {}", (*i.0).0, (*i.0).1, *i.1) 
    }
}

fn knapsack_inner(items: &Vec<Item>, ref mut memo: &mut HashMap<(usize, i32), usize>, weight: i32, index: usize) -> usize {
//     println!("-----------");
//     println!("taking a look at w {} i {}({} {}) with memo", weight, index, items[index - 1].value, items[index - 1].weight);
//     printMemo(memo);
    if (weight < 0) { 
       //  println!(" returning .. 0 because weight negative"); 
     panic!();
        return 0 }
    if (index == 0) { 
//         println!(" returning .. 0 vecause index is 0");
        return 0 
    }
//     if (index == 1) { if  weight >= items[0].weight as i32 { 
// //         println!(" returning .. {}", items[0].value);return items[0].value 
//     } else { 
//      panic!();
// //         println!(" returning .. 0 because index 1 and not enough weight left for first item");
//         return 0 } }

    if memo.get(&(index, weight)).is_none() {
        let o1 = knapsack_inner(&items, memo, weight, index - 1);
//         println!("got o1 for i {} w {}, it's {}", index, weight, o1);
        let o2 = if weight < items[index -1].weight as i32 { -1000000000} else {knapsack_inner(&items, memo, weight - items[index - 1].weight as i32, index - 1) as i32 };
//         println!("got o2 for i {} w {}, it's {}", index, weight, o2);

//         println!("about to make a decision for  w {} i {} ", weight, index);
//         println!("Inserted {} at i {}, w {}", std::cmp::max(o1 as i32, o2 + items[index - 1].value as i32), index, weight);

        memo.insert((index, weight), std::cmp::max(o1 as i32, o2 + items[index - 1].value as i32) as usize);
    }

    let res = *memo.get(&(index, weight)).unwrap();
//     println!(" returning .. {}", res);
    res
}

fn knapsack(items: &Vec<Item>, max_weight: usize) -> usize {

    let mut memo: HashMap<(usize, i32), usize> = HashMap::new();

    knapsack_inner(&items, &mut memo, max_weight as i32, items.len());
    // memo.insert((items.len(), max_weight as i32), 44);
    //printMemo(&memo);
    println!("items: {} mw {} ", items.len(), max_weight);
    println!("{}", *memo.get(&(items.len(), max_weight as i32)).unwrap());
    *memo.get(&(items.len(), max_weight as i32)).unwrap()
//     let mut stack: VecDeque<Item> = VecDeque::new();
//     let mut max_value: usize;
// 
//     stack.push_back(items[0]);
// 
//     while (!stack.is_empty()) {
//         let item = stack.pop_back();
//         stack.push_back(item)
//     }
// 
//     max_value
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
