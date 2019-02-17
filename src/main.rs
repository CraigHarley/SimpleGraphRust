use std::collections::HashMap;

fn main() {
    let matrix = HashMap::new();
    let step1 = add_edge(matrix, 1, 2);
    let step2 = add_edge(step1, 5, 1);
    let step3 = add_edge(step2, 5, 2);
    let step4 = add_edge(step3, 2, 6);

    for (&i, secondLevel) in step4.iter() {
        for (&j, &value) in secondLevel.iter() {
            println!("Calling {}, {}: {}", i, j, value);
        }
    }
}

fn add_edge(
    mut matrix: HashMap<u32, HashMap<u32, bool>>,
    i: u32,
    j: u32,
) -> HashMap<u32, HashMap<u32, bool>> {
    match matrix.get(&i) {
        Some(mut neighbors) => neighbors.insert(j, true),
        _ => {
            let mut neighbors = HashMap::new();
            neighbors.insert(j, true);
            matrix.insert(i, neighbors);
        }
    }

    match matrix.get(&j) {
        Some(mut neighbors) => neighbors.insert(i, true),
        _ => {
            let mut neighbors = HashMap::new();
            neighbors.insert(i, true);
            matrix.insert(j, neighbors);
        }
    }

    return matrix;
}