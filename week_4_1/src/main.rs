fn main() {
    let csv_matrix = read_distances("data/OCTO-Coding-Challenge-2024-Week-4-Part-1-input.txt")
        .expect("Failed to read distances");
    let distances = preprocess_distances(csv_matrix);
    let mst = kruskal(&distances);
    let total_distance: u32 = mst.iter().map(|&(_, _, distance)| distance).sum();
    println!("Total weight: {}", total_distance);
}

/// Considering that 0 distance between different cities, means there is no connection between them
fn preprocess_distances(distances: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = distances.clone();
    let n = result.len();
    for a in 0..n {
        for b in 0..n {
            if a != b && result[a][b] == 0 {
                result[a][b] = u32::MAX;
            }
        }
    }
    result
}

fn read_distances(filename: &str) -> std::io::Result<Vec<Vec<u32>>> {
    let mut distances = Vec::new();
    let content = std::fs::read_to_string(filename)?;
    for line in content.lines() {
        let line = line;
        let row: Vec<u32> = line
            .split(',')
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        distances.push(row);
    }
    Ok(distances)
}

fn kruskal(distances: &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let mut edges = Vec::new();
    let n = distances.len();
    for a in 0..n {
        for b in a + 1..n {
            edges.push((a, b, distances[a][b]));
        }
    }
    edges.sort_by_key(|&(_, _, weight)| weight);

    let mut parent = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];

    let mut mst = Vec::new();
    for (a, b, distance) in edges {
        if find(&mut parent, a) != find(&mut parent, b) {
            union(&mut parent, &mut rank, a, b);
            mst.push((a, b, distance));
        }
    }
    mst
}

fn find(parent: &mut Vec<usize>, city_index: usize) -> usize {
    if parent[city_index] != city_index {
        parent[city_index] = find(parent, parent[city_index]);
    }
    parent[city_index]
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(parent, a);
    let root_b = find(parent, b);
    if root_a != root_b {
        if rank[root_a] > rank[root_b] {
            parent[root_b] = root_a;
        } else if rank[root_a] < rank[root_b] {
            parent[root_a] = root_b;
        } else {
            parent[root_b] = root_a;
            rank[root_a] += 1;
        }
    }
}
//test result = 504,793
//with 0 distance edges = 50,026,102
//without 0 distance edges = 88,943,058

#[cfg(test)]
mod tests {
    use super::*;

    fn get_distances() -> Vec<Vec<u32>> {
        let filename = "data/OCTO-Coding-Challenge-2024-Week-4-Part-1-input.txt";
        let csv_matrix = read_distances(filename).expect("Failed to read distances");
        preprocess_distances(csv_matrix)
    }

    #[test]
    fn test_square_matrix() {
        let distances = get_distances();
        let n = distances.len();
        for row in &distances {
            assert_eq!(row.len(), n, "Matrix is not square");
        }
    }

    #[test]
    fn test_diagonal_zero() {
        let distances = get_distances();
        for i in 0..distances.len() {
            assert_eq!(distances[i][i], 0, "Diagonal element is not zero");
        }
    }

    #[test]
    fn test_symmetrical_matrix() {
        let distances = get_distances();
        let n = distances.len();
        for i in 0..n {
            for j in 0..n {
                assert_eq!(
                    distances[i][j], distances[j][i],
                    "Matrix is not symmetrical"
                );
            }
        }
    }

    #[test]
    fn test_non_zero_distances() {
        let distances = get_distances();
        let n = distances.len();
        let mut zero_distance_edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    if distances[i][j] == 0 {
                        zero_distance_edges.push((i, j));
                    }
                }
            }
        }

        assert!(
            zero_distance_edges.len() == 0,
            "There are zero distance edges: {:?}",
            zero_distance_edges
        );
    }
}
