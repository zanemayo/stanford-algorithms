extern crate time;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;
use std::collections::HashSet;

fn to_usize(s: String) -> usize {
    s.trim().parse::<usize>().expect("Not a usize")
}

fn to_bool(bool_str: &str) -> bool {
    if bool_str == "1" { true } else { false }
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

pub fn cluster(graph: &Vec<i32>) -> usize {
    let mut uf = create(graph.len());
    for j in 0..graph.len() {
        for k in j+1..graph.len() {
            if is_hamming_close(graph[j], graph[k]) {
                union(&mut uf, j, k);
            }
        }
    }
    get_num_clusters(&mut uf)
}

pub fn get_num_clusters(mut union_find: &mut Vec<Component>) -> usize {
    let uf_vec = union_find.iter()
        .map(|component| component.parent)
        .collect::<Vec<_>>();
    uf_vec.iter()
        .map(|v| find(&mut union_find, *v))
        .collect::<HashSet<usize>>().len()
}

fn main() {
    let graph = load("clustering.txt");
    let clustering_start_time = time::precise_time_ns();
    println!("{}", cluster(&graph));
    println!("Time to run clustering: {} ms", // 140s
        (time::precise_time_ns() - clustering_start_time) / 1_000_000);
    //6118
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

pub fn union(mut uf: &mut Vec<Component>, v1: usize, v2: usize) {
    let v1Parent = find(&mut uf, v1);
    let v2Parent = find(&mut uf, v2);
    if v1Parent == v2Parent { return }

    if uf[v1Parent].rank > uf[v2Parent].rank {
        uf[v2Parent].parent = uf[v1Parent].parent;
    }
    else if uf[v1Parent].rank < uf[v2Parent].rank {
        uf[v1Parent].parent = uf[v2Parent].parent;
    }
    else {
        uf[v2Parent].parent = uf[v1Parent].parent;
        uf[v1Parent].rank += 1;
    }
}

pub fn find(mut uf: &mut Vec<Component>, v: usize) -> usize {
    let parent = uf[v].parent;
    if parent != v {
        uf[v].parent = find(&mut uf, parent)
    }
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
        assert_eq!(2, find(&mut uf, 1));
    }
}


