extern crate time;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

pub fn load(filename: &str) -> (Vec<i32>, Vec<Vec<Vec<usize>>>) {
    let f = File::open(filename).expect("Can't open file");
    let mut reader = BufReader::new(f);
    let mut top_line = String::new();
    reader.read_line(&mut top_line).unwrap();

    let nodes = reader.lines()
        .map(|line|
             i32::from_str_radix(&line.unwrap().replace(" ", ""), 2).unwrap())
        .collect::<Vec<i32>>();

    let mut parts = vec![vec![vec![0; 0]; 256], vec![vec![0; 0]; 256], vec![vec![0; 0]; 256]];
    for (i, node) in nodes.iter().enumerate() {
        parts[0][(node >> 16) as usize].push(i);
        parts[1][((node >> 8) as u8) as usize].push(i);
        parts[2][(*node as u8) as usize].push(i);
    }
    (nodes, parts)
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

pub fn cluster(graph: &Vec<i32>, parts: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut uf = create(graph.len());
    for g in 0..parts.len() {
        let part = &parts[g];
        for h in 0..256 {
            for j in 0..part[h].len() {
                for k in j+1..part[h].len() {
                    if is_hamming_close(graph[part[h][j]], graph[part[h][k]]) {
                        union(&mut uf, part[h][j], part[h][k]);
                    }
                }
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
    let clustering_start_time = time::precise_time_ns();
    let (graph, parts) = load("clustering.txt");
    println!("Number of clusters: {}", cluster(&graph, &parts));
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
    let v1_parent = find(&mut uf, v1);
    let v2_parent = find(&mut uf, v2);
    if v1_parent == v2_parent { return }

    if uf[v1_parent].rank > uf[v2_parent].rank {
        uf[v2_parent].parent = uf[v1_parent].parent;
    }
    else if uf[v1_parent].rank < uf[v2_parent].rank {
        uf[v1_parent].parent = uf[v2_parent].parent;
    }
    else {
        uf[v2_parent].parent = uf[v1_parent].parent;
        uf[v1_parent].rank += 1;
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

    fn get_test_graph() -> Vec<i32>{
        vec!(11_184_810, 986_895, 9_375_503)
    }

    fn get_test_graph_with_arrays() -> (Vec<i32>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>){
        let mut top = vec![vec![0; 0]; 256];
        top[170].push(0);
        top[15].push(1);
        top[143].push(2);
        let mut middle = vec![vec![0; 0]; 256];
        middle[170].push(0);
        middle[15].push(1);
        middle[15].push(2);
        let mut lower = vec![vec![0; 0]; 256];
        lower[170].push(0);
        lower[15].push(1);
        lower[15].push(2);
        (get_test_graph(), top, middle, lower)
    }

    #[test]
    fn graph_loads() {
        let graph = load("test.txt");
        assert_eq!(get_test_graph(), graph);
    }

    #[test]
    fn graph_loads_into_hashmap() {
        let (graph, top, middle, lower) = load2("test.txt");
        let (graph_expected, top_expected, middle_expected, lower_expected) = get_test_graph_with_arrays();
        assert_eq!(graph_expected, graph);
        assert_eq!(top_expected[170], top[170]);
        assert_eq!(top_expected[15], top[15]);
        assert_eq!(top_expected[143], top[143]);
        assert_eq!(middle_expected[170], middle[170]);
        assert_eq!(middle_expected[15], middle[15]);
        assert_eq!(lower_expected[170], lower[170]);
        assert_eq!(lower_expected[15], lower[15]);
    }

//     #[test]
//     fn hamming_distance() {
//         let graph = load("test.txt");
//         assert_eq!(12, get_hamming_distance(graph[0], graph[1]));
//         assert_eq!(0, get_hamming_distance(14734287, 14734287));
//         assert_eq!(true, is_hamming_close(14734287, 14734287));
//     }

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

//     #[test]
//     fn union_find_union() {
//         let mut uf = create(3);
//         union(&mut uf, 1, 2);
//         assert_eq!(vec! (Component { parent: 0, rank: 0 },
//                          Component { parent: 2, rank: 1 },
//                          Component { parent: 2, rank: 1 }),
//                          uf);
//         union(&mut uf, 0, 1);
//         assert_eq!(vec! (Component { parent: 2, rank: 1 },
//                          Component { parent: 2, rank: 1 },
//                          Component { parent: 2, rank: 1 }),
//                          uf);
//         union(&mut uf, 0, 2);
//         assert_eq!(vec! (Component { parent: 2, rank: 1 },
//                          Component { parent: 2, rank: 1 },
//                          Component { parent: 2, rank: 1 }),
//                          uf);
//     }
// 
//     #[test]
//     fn union_find_find() {
//         let mut uf = create(3);
//         union(&mut uf, 1, 2);
//         assert_eq!(2, find(&mut uf, 1));
//     }
}


