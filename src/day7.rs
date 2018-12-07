#![feature(extern_prelude)]
use petgraph::prelude::*;
use petgraph::EdgeType;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

use petgraph as pg;

use petgraph::algo::{
    dominators, has_path_connecting, is_cyclic_undirected, is_isomorphic_matching,
    min_spanning_tree,
};

use petgraph::graph::node_index as n;
use petgraph::graph::IndexType;

use petgraph::algo::{astar, dijkstra, DfsSpace};
use petgraph::visit::{
    IntoNeighbors, IntoNodeIdentifiers, NodeFiltered, Reversed, Topo, VisitMap, Walker,
};

use petgraph::dot::Dot;

use std::collections::HashSet;
use std::hash::Hash;

use petgraph::prelude::*;
// use petgraph::EdgeType;

// use petgraph as pg;

// use petgraph::algo::{
//     dominators, has_path_connecting, is_cyclic_undirected, is_isomorphic_matching,
//     min_spanning_tree,
// };

// use petgraph::graph::node_index as n;
// use petgraph::graph::IndexType;

// use petgraph::algo::{astar, dijkstra, DfsSpace};
// use petgraph::visit::{
//     IntoNeighbors, IntoNodeIdentifiers, NodeFiltered, Reversed, Topo, VisitMap, Walker,
// };

// use petgraph::dot::Dot;

pub fn part1() -> Result<String, Error> {
    let f = File::open("resources/day7.txt")?;
    let f = BufReader::new(f);
    let mut gr = Graph::<_, _>::new();
    // let vec: Vec<(&str, &str)> = Vec::new();
    let mut node_list = HashMap::new();
    let mut node_indicies = HashMap::new();

    for line in f.lines() {
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();

        let line_split = line_split.split(" ").collect::<Vec<&str>>();
        let edge = {
            (
                if !node_list.contains_key(line_split[1]) {
                    let node = gr.add_node(line_split[1].to_owned());
                    node_list.insert(line_split[1].to_owned(), node);
                    node_indicies.insert(node, line_split[1].to_owned());
                    node
                } else {
                    *node_list.get(line_split[1]).unwrap()
                },
                if !node_list.contains_key(line_split[7]) {
                    let node = gr.add_node(line_split[7].to_owned());
                    node_list.insert(line_split[7].to_owned(), node);
                    node_indicies.insert(node, line_split[7].to_owned());
                    node
                } else {
                    *node_list.get(line_split[7]).unwrap()
                },
            )
        };

        gr.add_edge(
            edge.0,
            edge.1,
            line_split[7].to_owned().chars().next().unwrap() as u32,
        );
    }
    // let mut state = DfsSpace::default();
    // No documentation for how to implement toposort :(
    let order = pg::algo::toposort(&gr, None).unwrap();
    println!("{:?} {:?}", gr, node_indicies);
    let mut return_string = "".to_string();
    // return_string.push_str(&node_indicies.get(&order[0]).unwrap());

    for node_index in order {
        return_string.push_str(&node_indicies.get(&node_index).unwrap());
        for neighbor in gr.neighbors(node_index) {
            println!("{:?}", &node_indicies.get(&neighbor).unwrap());
        }
    }

    // for neighbor in gr.neighbors(node_index) {
    //     println!("{:?}", &node_indicies.get(&neighbor).unwrap());
    // }
    // let first_node = gr.neighbors(order[0]).collect();

    // first_node.sort_unstable_by(|left, right| left.0.timestamp().cmp(&right.0.timestamp()));
    // let mut neighbor_vec = Vec::new();
    // for neighbor in gr.neighbors(order[0]) {
    //     neighbor_vec.push((neighbor, node_indicies.get(&neighbor).cloned().unwrap()));
    // }

    // neighbor_vec.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    // println!("{:?}", neighbor_vec);
    // println!("{}", Dot::new(&gr));

    // println!("{:?}", order);
    Ok(return_string)
}

pub fn part2() -> Result<i32, Error> {
    let f = File::open("resources/day7.txt")?;
    let f = BufReader::new(f);
    Ok(0)
}
