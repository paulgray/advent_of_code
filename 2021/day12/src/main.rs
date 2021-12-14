use std::collections::HashMap;

struct Graph {
    nodes: Vec<u32>,
    edges: Vec<(u32, u32)>,
}

fn find_paths(
    n1: u32,
    n2: u32,
    visited: &mut Vec<u32>,
    multivisit: &Vec<u32>,
    graph: &Graph,
) -> u32 {
    let mut sum = 0;

    // get all possible edges from n1
    for (e1, e2) in &graph.edges {
        if *e1 == n1 {
            // we either have not visited the node or it's okay to visit it
            // multiple times
            if !visited.contains(e2) || multivisit.contains(e2) {
                let mut new_visited = visited.clone();
                new_visited.push(*e2);

                // we reached the end
                if *e2 == n2 {
                    sum += 1;
                } else {
                    sum += find_paths(*e2, n2, &mut new_visited, &multivisit, &graph);
                }
            }
        }
    }

    return sum;
}
fn star1(input: &str) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut ids: HashMap<&str, u32> = HashMap::new();
    let mut multivisit: Vec<u32> = Vec::new();
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    // assign IDs to nodes
    for line in lines {
        let connection = line.split("-").collect::<Vec<&str>>();
        let start = connection[0];
        let end = connection[1];

        if !ids.contains_key(start) {
            ids.insert(start, ids.len() as u32);
        }
        if !ids.contains_key(end) {
            ids.insert(end, ids.len() as u32);
        }

        // add nodes to the graph
        let start_id = ids.get(start).unwrap();
        if !graph.nodes.contains(start_id) {
            graph.nodes.push(*start_id);
        }

        let end_id = ids.get(end).unwrap();
        if !graph.nodes.contains(end_id) {
            graph.nodes.push(*end_id);
        }

        // if the node name is uppercased, it can be visited
        // multiple times
        if start
            .chars()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap()
            .is_uppercase()
            && !multivisit.contains(start_id)
        {
            multivisit.push(*start_id);
        }

        if end
            .chars()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap()
            .is_uppercase()
            && !multivisit.contains(end_id)
        {
            multivisit.push(*end_id);
        }

        // add edges to the graph
        // for simplicity add edges in both directions
        let edge1 = (*start_id, *end_id);
        let edge2 = (*end_id, *start_id);
        graph.edges.push(edge1);
        graph.edges.push(edge2);
    }

    // build all the paths from "start" to "end"
    let start_id = ids.get("start").unwrap();
    let end_id = ids.get("end").unwrap();
    let mut visited: Vec<u32> = Vec::new();
    visited.push(*start_id);

    let paths = find_paths(*start_id, *end_id, &mut visited, &multivisit, &graph);
    println!("Paths found: {}", paths);
}

fn main() {
    let contents =
        std::fs::read_to_string("test3").expect("Something went wrong when reading the input file");

    star1(&contents);
}
