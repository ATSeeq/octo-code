use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

type Graph = HashMap<String, HashMap<String, f64>>;

fn main() -> Result<(), Box<dyn Error>> {
    let graph: Graph =
        parse_graph_from_json_file("data/OCTO-Coding-Challenge-2024-Week-4-Part-2-input.txt")?;

    let max_flow = edmonds_karp(&graph, "A", "Z");
    println!("Max flow: {:.2}", max_flow);
    Ok(())
}

fn parse_graph_from_json_file(file_path: &str) -> Result<Graph, Box<dyn Error>> {
    let json_string = read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&json_string)?;

    let graph = json
        .as_object()
        .ok_or("Expected a JSON object at the top level")?
        .iter()
        .map(|(node, edges)| {
            let neighbors = edges
                .as_object()
                .ok_or("Expected a JSON object for edges")?
                .iter()
                .filter_map(|(neighbor, capacity)| {
                    capacity.as_f64().map(|cap| (neighbor.clone(), cap))
                })
                .collect::<HashMap<String, f64>>();

            Ok((node.clone(), neighbors))
        })
        .collect::<Result<Graph, Box<dyn Error>>>()?;

    Ok(graph)
}

fn bfs(graph: &Graph, source: &str, sink: &str, parent: &mut HashMap<String, String>) -> bool {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(source.to_string());
    visited.insert(source.to_string(), true);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, &capacity) in neighbors {
                if !visited.get(neighbor).unwrap_or(&false) && capacity > 0.0 {
                    queue.push_back(neighbor.clone());
                    visited.insert(neighbor.clone(), true);
                    parent.insert(neighbor.clone(), current.clone());

                    if neighbor == sink {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn edmonds_karp(graph: &Graph, source: &str, sink: &str) -> f64 {
    let mut residual_graph = graph.clone();
    let mut max_flow = 0.0;

    loop {
        let mut parent: HashMap<String, String> = HashMap::new();
        if !bfs(&residual_graph, source, sink, &mut parent) {
            break;
        }

        let mut path_flow = f64::MAX;
        let mut s = sink.to_string();
        while s != source {
            let t = parent.get(&s).unwrap();
            path_flow = path_flow.min(residual_graph.get(t).unwrap().get(&s).unwrap().clone());
            s = t.clone();
        }

        s = sink.to_string();
        while s != source {
            let t = parent.get(&s).unwrap();
            let edge_capacity = residual_graph.get_mut(t).unwrap().get_mut(&s).unwrap();
            *edge_capacity -= path_flow;

             // Handle case where reverse edge doesn't exist yet
             if let Some(reverse_edge) = residual_graph.get_mut(&s) {
                if let Some(rev_capacity) = reverse_edge.get_mut(t) {
                    *rev_capacity += path_flow;
                } else {
                    reverse_edge.insert(t.clone(),path_flow);
                }
            } else {
                 let mut new_edge = HashMap::new();
                 new_edge.insert(t.clone(), path_flow);
                residual_graph.insert(s.clone(), new_edge);
            }

            s = t.clone();
        }

        max_flow += path_flow;
    }

    max_flow
}

// 15.64
