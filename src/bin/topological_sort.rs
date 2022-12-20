use std::io::stdin;
use std::collections::{BTreeSet, VecDeque};

fn main() {
    println!("Topological sort on a graph\nEnter number of nodes:");
    let mut num_nodes = String::new();
    stdin().read_line(&mut num_nodes)
        .expect("Could not read number of nodes");
    let num_nodes: usize = num_nodes.trim().parse()
        .expect("Could not parse as usize");

    println!("Enter number of edges:");
    let mut num_edges = String::new();
    stdin().read_line(&mut num_edges)
        .expect("Could not read number of edges");
    let num_edges: usize = num_edges.trim().parse()
        .expect("Could not parse as usize");

    let mut graph = vec![BTreeSet::<usize>::new(); num_nodes];
    let mut in_degrees = vec![0u8; num_nodes];
    for _ in 0..num_edges {
        println!("Enter an edge <from to>:");
        let mut edge = String::new();
        stdin().read_line(&mut edge)
            .expect("Could not read edge");
        let edge = edge.trim().split(' ').collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string().parse::<usize>()
                .expect("Could not parse as usize")
            )
            .collect::<Vec<_>>();

        let (from, to) = (&edge[0], &edge[1]);
        in_degrees[*to] += 1;
        graph[*from].insert(*to);
    }

    println!("Topological sort: {:?}", khan_algorithm(&graph, &mut in_degrees));
}

fn khan_algorithm(graph: &Vec<BTreeSet<usize>>, in_degrees: &mut Vec<u8>) -> Vec<usize> {
    let mut queue = VecDeque::<usize>::new();
    for entry in 0..in_degrees.len() {
        if in_degrees[entry] == 0 {
            queue.push_back(entry);
        }
    }

    let mut sorted = Vec::<usize>::new();
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        sorted.push(current);

        if !graph[current].is_empty() {
            for node in &graph[current] {
                in_degrees[*node] -= 1;
                if in_degrees[*node] == 0 {
                    queue.push_back(*node);
                }
            }
        }
    }

    sorted
}