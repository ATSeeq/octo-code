use serde_json as json;
use std::collections::HashMap;
use std::fs;

type Graph = HashMap<String, HashMap<String, f64>>;
type Memo<'a> = HashMap<&'a str, usize>;

fn main() {
    let data =
        fs::read_to_string("data/OCTO-Coding-Challenge-2024-Week-3-Part-1-input.txt").unwrap();
    let graph: Graph = json::from_str(&data).unwrap();

    let mut memo = Memo::new();
    let max_word_count = dfs(&graph, "START", &mut memo);
    println!("Max word count: {}", max_word_count);
}

fn dfs<'a>(graph: &'a Graph, node: &'a str, memo: &mut Memo<'a>) -> usize {
    if let Some(&result) = memo.get(node) {
        return result;
    }

    if node == "END" {
        return 0;
    }

    let max_length = graph.get(node).map_or(0, |neighbors| {
        neighbors
            .keys()
            .map(|neighbor| dfs(graph, neighbor, memo))
            .max()
            .unwrap()
    });

    let length = if node == "START" {
        max_length
    } else {
        max_length + 1
    };

    memo.insert(node, length);
    length
}
// 23
