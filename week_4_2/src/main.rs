use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

type Graph = HashMap<String, HashMap<String, f64>>;

fn main() -> Result<(), Box<dyn Error>> {
    let graph = load_graph_from_file("data/OCTO-Coding-Challenge-2024-Week-4-Part-2-input.txt")?;

    let max_flow = edmonds_karp(&graph, "A", "Z");
    println!("Max flow: {:.2}", max_flow);
    Ok(())
}

fn load_graph_from_file(file_path: &str) -> Result<Graph, Box<dyn Error>> {
    let json_string = read_to_string(file_path)?;
    parse_graph_from_json(&json_string)
}

fn parse_graph_from_json(json_string: &str) -> Result<Graph, Box<dyn Error>> {
    let json: Value = serde_json::from_str(json_string)?;

    let graph = json
        .as_object()
        .ok_or("Expected a JSON object at the top level")?
        .iter()
        .map(|(node, edges)| parse_edges(node, edges))
        .collect::<Result<Graph, Box<dyn Error>>>()?;

    Ok(graph)
}

fn parse_edges(
    node: &String,
    edges: &Value,
) -> Result<(String, HashMap<String, f64>), Box<dyn Error>> {
    let neighbors = edges
        .as_object()
        .ok_or_else(|| format!("Expected a JSON object for edges of node: {}", node))?
        .iter()
        .filter_map(|(neighbor, capacity)| capacity.as_f64().map(|cap| (neighbor.clone(), cap)))
        .collect::<HashMap<String, f64>>();
    Ok((node.clone(), neighbors))
}

fn bfs(graph: &Graph, source: &str, sink: &str, parent: &mut HashMap<String, String>) -> bool {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(source.to_string());
    visited.insert(source.to_string(), true);

    while let Some(current) = queue.pop_front() {
        if current == sink {
            return true;
        }

        if let Some(neighbors) = graph.get(&current) {
            for (neighbor, capacity) in neighbors {
                if !visited.get(neighbor).unwrap_or(&false) && *capacity > 0.0 {
                    queue.push_back(neighbor.clone());
                    visited.insert(neighbor.clone(), true);
                    parent.insert(neighbor.clone(), current.clone());
                }
            }
        }
    }
    false
}

fn edmonds_karp(graph: &Graph, source: &str, sink: &str) -> f64 {
    let mut residual_graph = graph.clone();
    let mut max_flow = 0.0;

    while let Some(parent) = find_augmenting_path(&residual_graph, source, sink) {
        let path_flow = calculate_path_flow(&residual_graph, &parent, sink, source);

        if path_flow == 0.0 {
            break;
        }
        update_residual_graph(&mut residual_graph, &parent, sink, path_flow);
        max_flow += path_flow;
    }
    max_flow
}

fn find_augmenting_path(
    residual_graph: &Graph,
    source: &str,
    sink: &str,
) -> Option<HashMap<String, String>> {
    let mut parent: HashMap<String, String> = HashMap::new();
    if bfs(residual_graph, source, sink, &mut parent) {
        Some(parent)
    } else {
        None
    }
}

fn calculate_path_flow(
    residual_graph: &Graph,
    parent: &HashMap<String, String>,
    sink: &str,
    source: &str,
) -> f64 {
    let mut path_flow = f64::MAX;
    let mut current_node = sink.to_string();
    while current_node != source {
        let previous_node = parent
            .get(&current_node)
            .ok_or_else(|| format!("No parent node found for {}", current_node))
            .unwrap();

        let capacity = residual_graph
            .get(previous_node)
            .and_then(|neighbors| neighbors.get(&current_node))
            .ok_or_else(|| {
                format!(
                    "No capacity found for edge from {} to {}",
                    previous_node, current_node
                )
            })
            .unwrap();

        path_flow = path_flow.min(*capacity);
        current_node = previous_node.clone();
    }
    path_flow
}

fn update_residual_graph(
    residual_graph: &mut Graph,
    parent: &HashMap<String, String>,
    sink: &str,
    path_flow: f64,
) {
    let mut current_node = sink.to_string();
    while let Some(previous_node) = parent.get(&current_node) {
        // Update the capacity of the forward edge
        if let Some(neighbors) = residual_graph.get_mut(previous_node) {
            if let Some(capacity) = neighbors.get_mut(&current_node) {
                *capacity -= path_flow;
            }
        }

        // Update the capacity of the reverse edge
        let reverse_edge = residual_graph
            .entry(current_node.clone())
            .or_insert_with(HashMap::new);
        *reverse_edge.entry(previous_node.clone()).or_insert(0.0) += path_flow;

        current_node = previous_node.clone();
    }
}
