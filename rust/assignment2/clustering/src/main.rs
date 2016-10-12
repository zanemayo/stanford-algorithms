use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;
use std::collections::HashSet;

fn to_usize(s: String) -> usize {
    s.trim().parse::<usize>().expect("Not a usize")
}

// pub struct Graph {
//     v: usize,
//     nodes: Vec<Vec<bool>>,
// }
// 
// impl Graph {

fn to_bool(bool_str: &str) -> bool {
    if bool_str == "1" { true } else { false }
}

pub fn load2(filename: &str) -> Vec<Vec<bool>> {
    let f = File::open(filename).expect("Can't open file");
    let mut reader = BufReader::new(f);
    let mut top_line = String::new();
    reader.read_line(&mut top_line).unwrap();
    let v = to_usize(top_line);

    reader.lines()
        .map(|line|
             line.unwrap().split(" ")
             .map(|b| to_bool(&b))
             .collect())
        .collect()
}

pub fn load(filename: &str) -> Vec<i32> {
    let f = File::open(filename).expect("Can't open file");
    let mut reader = BufReader::new(f);
    let mut top_line = String::new();
    reader.read_line(&mut top_line).unwrap();
    let v = to_usize(top_line);

    reader.lines()
        .map(|line|
             i32::from_str_radix(&line.unwrap().replace(" ", ""), 2).unwrap())
        .collect()
}


pub fn get_hamming_distance(v1: i32, v2: i32) -> i32 {
    let mut xor = v1 ^ v2;
    let mut count = 0;
    while xor > 0 {
        count += 1;
        xor = xor & (xor - 1)
    }
    count
}

pub fn is_hamming_close(v1: i32, v2: i32) -> bool {
    let mut xor = v1 ^ v2;
    let mut count = 0;
    while xor > 0 {
        count += 1;
        if count > 2 { return false }
        xor = xor & (xor - 1)
    }
    true
}

pub fn get_hamming_distance2(v1: &Vec<bool>, v2: &Vec<bool>) -> i32 {
    let mut dist = 0;
    for i in 0..v1.len() {
        if (v1[i] == true && v2[i] == false) || (v1[i] == false && v2[i] == true) {
            dist += 1;
        }
    }
    dist
}

pub fn cluster(graph: &Vec<i32>) -> usize {
    let mut uf = create(graph.len());
    for j in 0..graph.len() {
//        println!("{}", j);
        for k in j..graph.len() {
            //if get_hamming_distance(graph[j], graph[k]) < 23  {
            if is_hamming_close(graph[j], graph[k]) {
                union(&mut uf, j, k);
            }
//             else {
//                 println!("{} {} {} not close", graph[j], graph[k], get_hamming_distance(graph[j], graph[k]))
//             }
        }
    }
    get_num_clusters(&uf)
}

pub fn cluster2(graph: &Vec<Vec<bool>>) -> usize {
    let mut uf = create(graph.len());
    for i in 0..23 {
        println!("{}", i);
        for j in 0..graph.len() {

        println!("{}", j);
            for k in 0..graph.len() {
                if get_hamming_distance2(&graph[j], &graph[k]) == i  {
                   union(&mut uf, j, k)
                }
            }
        }
    }
    get_num_clusters(&uf)
}

pub fn get_num_clusters(union_find: &Vec<Component>) -> usize {
    union_find.iter().map(|component| component.parent)
        .collect::<HashSet<usize>>().len()
}

fn main() {
    println!("hello world");
    let graph = load("clustering.txt");
    println!("{}", cluster(&graph));
}

#[derive(PartialEq, Eq, Debug)]
pub struct Component {
    parent: usize,
    rank: usize,
}

pub fn create(size: usize) -> Vec<Component> {
    (0..size)
        .map(|i| Component { parent: i, rank: 0 })
        .collect()
}

pub fn union(uf: &mut Vec<Component>, v1: usize, v2: usize) {
   let (rename_from, rename_to) = if uf[v1].rank > uf[v2].rank { (uf[v2].parent, uf[v1].parent) } else { (uf[v1].parent, uf[v2].parent) };
   if rename_from == rename_to { return; }
   let new_rank = uf[rename_to].rank + if uf[rename_from].rank == uf[rename_to].rank { 1 } else { 0 };
   for v in 0..uf.len() {
       if uf[v].parent == rename_from { uf[v].parent = rename_to; uf[v].rank = new_rank  }
       if uf[v].parent == rename_to { uf[v].rank = new_rank }
   }
}

pub fn find(uf: &Vec<Component>, v: usize) -> usize {
    uf[v].parent
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_graph2() -> Vec<Vec<bool>>{
        vec!(
            vec! [true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false],
            vec! [false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true],
            vec! [true, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true])
    }

    fn get_test_graph() -> Vec<i32>{
        vec!(1,2,3)
    }

    #[test]
    fn graph_loads() {
        let graph = load("test.txt");
        assert_eq!(get_test_graph(), graph);
    }

    #[test]
    fn hamming_distance() {
        let graph = load("test.txt");
        assert_eq!(12, get_hamming_distance(graph[0], graph[1]));
        assert_eq!(0, get_hamming_distance(14734287, 14734287));
        assert_eq!(true, is_hamming_close(14734287, 14734287));
        

    }

//     #[test]
//     fn clustering() {
//         let graph = load("clustering.txt");
//         assert_eq!(5, cluster(&graph))
//     }

    #[test]
    fn union_find_create() {
        let uf = create(3);
        assert_eq!(3, uf.len());
        assert_eq!(0, uf[0].parent);
        assert_eq!(1, uf[1].parent);
        assert_eq!(2, uf[2].parent);
    }

    #[test]
    fn union_find_union() {
        let mut uf = create(3);
        union(&mut uf, 1, 2);
        assert_eq!(vec! (Component { parent: 0, rank: 0 },
                         Component { parent: 2, rank: 1 },
                         Component { parent: 2, rank: 1 }),
                         uf);
        union(&mut uf, 0, 1);
        assert_eq!(vec! (Component { parent: 2, rank: 1 },
                         Component { parent: 2, rank: 1 },
                         Component { parent: 2, rank: 1 }),
                         uf);
        union(&mut uf, 0, 2);
        assert_eq!(vec! (Component { parent: 2, rank: 1 },
                         Component { parent: 2, rank: 1 },
                         Component { parent: 2, rank: 1 }),
                         uf);
    }

    #[test]
    fn union_find_find() {
        let mut uf = create(3);
        union(&mut uf, 1, 2);
        assert_eq!(2, find(&uf, 1));
    }
}


