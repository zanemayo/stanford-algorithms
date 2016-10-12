extern crate time;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt;

fn to_int(s: &str) -> i32 {
    s.trim().parse::<i32>().expect("Not an int")
}

fn to_usize(s: &str) -> usize {
    s.trim().parse::<usize>().expect("Not a usize")
}

fn line_to_edge(line: String) -> Edge {
    let l: Vec<&str> = line.split(" ").collect();
    Edge { 
        v1: to_usize(l[0]), 
        v2: to_usize(l[1]), 
        weight: to_int(l[2])
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v1: {}, v2: {}, weight: {}", self.v1, self.v2, self.weight)
    }
}


struct Edge {
    v1: usize,
    v2: usize,
    weight: i32,
}

struct Graph {
    v: usize,
    e: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn load(filename: &str) -> Graph {
        let f = File::open(filename).expect("Can't open");
        let mut reader = BufReader::new(f);

        let mut top_line = String::new();
        reader.read_line(&mut top_line).expect("Can't read");
        println!("{:?}", top_line);
        let l: Vec<&str> = top_line.split(" ").collect();
        let v = to_usize(l[0]);
        let e = to_usize(l[1]);

        let edges: Vec<_> = reader.lines()
            .map(|line| line_to_edge(line.unwrap()))
            .collect();
        Graph { v: v, e: e, edges: edges }
    }
}

fn get_primms_cost(graph: &Graph) -> i32 {
    let mut in_graph = vec![false; (graph.v + 1) as usize];
    in_graph[1] = true;
    let mut cost: i32 = 0;

    for _ in 1..graph.v {
        let mut min_cost = i32::max_value();
        let mut min_edge = &graph.edges[0];
        for edge in &graph.edges {
            if edge.weight < min_cost &&
               in_graph[edge.v1] != in_graph[edge.v2] {
                min_cost = edge.weight;
                min_edge = edge;
            }
        }

        cost += min_cost;
        in_graph[min_edge.v1] = true;
        in_graph[min_edge.v2] = true;
    }
    cost
}

fn main() {
    let graph = &Graph::load("edges.txt");
    let program_start_time = time::precise_time_ns();
    let cost = get_primms_cost(&graph);
    println!("Time to run entire program: {} ms", // 4ms
        (time::precise_time_ns() - program_start_time) / 1_000_000);
    println!("Cost {}", cost);
   
}

//    let mut data = String::new();
//    let mut f = File::open(filename).expect("Can't open");
//    f.read_to_string(&mut data).expect("Can't read");


//        for i in 0..graph.e {
//            if in_graph[graph.edges[i].v1] != in_graph[graph.edges[i].v2] &&
//               graph.edges[i].weight < min_cost {
//                min_cost = graph.edges[i].weight;
//                min_edge = i;
//            }
//        }

//        in_graph[graph.edges[min_edge].v1] = true;
//        in_graph[graph.edges[min_edge].v2] = true;
