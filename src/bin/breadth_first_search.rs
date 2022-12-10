use std::io::stdin;
use std::collections::vec_deque::VecDeque;

fn main() {
    let mut num_nodes = String::new();
    println!("Enter the number of nodes:");
    stdin().read_line(&mut num_nodes).expect("Cannot read number of nodes");
    num_nodes.remove(num_nodes.len() - 1);
    let num_nodes = num_nodes.parse::<usize>().expect("Cannot parse as i32");

    let mut num_edges = String::new();
    println!("Enter the number of edges:");
    stdin().read_line(&mut num_edges).expect("Cannot read number of edges");
    num_edges.remove(num_edges.len() - 1);
    let num_edges = num_edges.parse::<usize>().expect("Cannot parse as i32");

    let mut graph = vec![Vec::<i32>::new(); num_nodes];
    for i in 0..num_nodes { graph[i] = Vec::new(); }

    for _ in 0..num_edges {
        println!("Enter an edge <to: i32, from: i32>");
        let mut edge = String::new();
        stdin().read_line(&mut edge).expect("Cannot read edge");
        edge.remove(edge.len() - 1);

        let edge = edge.as_str().split(',').collect::<Vec<&str>>()
            .iter().map(|&x| x.to_string().replace(" ","").parse::<i32>()
            .expect("Cannot parse as i32")).collect::<Vec<i32>>();
        graph[edge[0] as usize].push(edge[1]);
    }

    println!("Here is the graph you entered: {:?}", graph);
    println!("Enter the node index (root) to begin the BFS algorithm: ");
    let mut root = String::new();
    stdin().read_line(&mut root).expect("Error reading root");
    root.remove(root.len() - 1);
    let root = root.parse::<usize>().expect("Cannot parse as usize");

    println!("BFS result: ");
    println!("{:?}", breadth_first_search(&graph, root));
}

fn breadth_first_search(graph: &Vec<Vec<i32>>, root: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::<i32>::new();

    queue.push_back(root as i32);
    visited[root] = true;

    while !queue.is_empty() {
        let local_root = queue.pop_front().unwrap();
        result.push(local_root);

        for node in &graph[local_root as usize] {
            if !visited[*node as usize] {
                visited[*node as usize] = true;
                queue.push_back(*node);
            }
        }
    }

    result
}
