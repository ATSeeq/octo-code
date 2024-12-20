use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// Define a type alias for the transition probabilities
type Graph = HashMap<String, HashMap<String, f64>>;
type ExpectedLengths = HashMap<String, f64>;

fn calculate_average_length_dp(json_file_path: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // 1. Read the JSON file
    let mut file = File::open(json_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 2. Parse the JSON data into a HashMap
    let transition_probabilities: Graph = serde_json::from_str(&contents)?;

    // 3. Create the list of nodes
    let mut nodes: Vec<String> = transition_probabilities.keys().cloned().collect();
    if !nodes.contains(&"END".to_string()) {
        nodes.push("END".to_string());
    }

    // 4. Initialize expected lengths
    let mut expected_lengths: ExpectedLengths =
        nodes.iter().map(|node| (node.clone(), 0.0)).collect();

    // 5. Iterate until convergence
    let mut converged = false;
    while !converged {
        converged = true;

        for node in &nodes {
            if node == "END" {
                continue; // Skip the END node
            }

            let mut new_expected_length = 0.0;

            if let Some(next_nodes) = transition_probabilities.get(node) {
                for (next_node, prob) in next_nodes {
                    if node != "START" {
                        new_expected_length +=
                            prob * (1.0 + expected_lengths.get(next_node).unwrap());
                    } else {
                        new_expected_length += prob * expected_lengths.get(next_node).unwrap();
                    }
                }
            }

            if (new_expected_length - expected_lengths.get(node).unwrap_or(&0.0)).abs() > 1e-8 {
                expected_lengths.insert(node.clone(), new_expected_length);
                converged = false;
            }
        }
    }

    // 6. Return the expected length from the START node
    Ok(*expected_lengths.get("START").unwrap_or(&0.0))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_file_path = "data/OCTO-Coding-Challenge-2024-Week-3-Part-2-input.txt";
    let avg_length_iter_dp = calculate_average_length_dp(json_file_path)?;
    println!("The average sentence length is: {:.2}", avg_length_iter_dp);

    Ok(())
}

// 25.84
