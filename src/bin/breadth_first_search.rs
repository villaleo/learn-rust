use std::io::stdin;
use std::collections::vec_deque::VecDeque;
use std::str::FromStr;

/// Reads a line from [stdin] and attempts to parses it as [T], where T
/// implements the [FromStr] trait.
///
/// Because `read_value` is so general, type annotations or the 'turbofish' syntax
/// must be used to help the inference algorithm understand specifically which type
/// you're trying to parse into.
fn read_value<T: FromStr>() -> Result<T, T::Err> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Error reading line");
    buffer.trim().parse::<T>()
}

fn main() {
    println!("Enter the number of nodes:");
    let num_nodes = read_value::<usize>().unwrap();
    println!("Enter the number of edges:");
    let num_edges = read_value::<usize>().unwrap();

    let mut graph = vec![Vec::<i32>::new(); num_nodes];
    for _ in 0..num_edges {
        println!("Enter an edge <to: i32, from: i32>");
        let edge = read_value::<String>().unwrap().as_str().split(',')
            .collect::<Vec<&str>>().iter()
            .map(|&x| x.to_string().replace(" ","").parse::<i32>().expect("Cannot parse as i32"))
            .collect::<Vec<i32>>();

        graph[edge[0] as usize].push(edge[1]);
    }

    println!("Here is the graph you entered: {:?}", graph);
    println!("Enter the node index (root) to begin the BFS algorithm: ");
    println!("BFS result: ");
    let root = read_value::<usize>().unwrap();
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
