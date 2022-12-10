use std::io::stdin;

fn main() {
    let mut num_nodes = String::new();
    println!("Enter the number of nodes:");
    stdin().read_line(&mut num_nodes).expect("Cannot read number of nodes");
    num_nodes.remove(num_nodes.len() - 1);
    let num_nodes = num_nodes.parse::<usize>().expect("Cannot parse as i32");

    let mut graph = vec![Vec::<i32>::new(); num_nodes];
    for i in 0..num_nodes { graph[i] = Vec::new(); }

    for _ in 0..num_nodes {
        println!("Enter an edge <to: i32, from: i32>, where both values are in range [0, number of nodes]:");
        let mut edge = String::new();
        stdin().read_line(&mut edge).expect("Cannot read edge");
        edge.remove(edge.len() - 1);

        let edge = edge.as_str().split(',').collect::<Vec<&str>>()
            .iter().map(|&x| x.to_string().parse::<i32>()
            .expect("Cannot parse as i32")).collect::<Vec<i32>>();
        graph[edge[0] as usize].push(edge[1]);
    }

    println!("{:?}", graph);
}
