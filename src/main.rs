use std::collections::HashMap;

fn main() {
    let matrix = HashMap::new();
    let matrix = add_edge(matrix, 1, 2);
    let matrix = add_edge(matrix, 5, 1);
    let matrix = add_edge(matrix, 5, 2);
    let matrix = add_edge(matrix, 2, 6);

    for (&i, second_level) in matrix.iter() {
        for (&j, &value) in second_level.iter() {
            println!("Calling {}, {}: {}", i, j, value);
        }
    }
}

fn add_edge(
    mut matrix: HashMap<u32, HashMap<u32, bool>>,
    i: u32,
    j: u32,
) -> HashMap<u32, HashMap<u32, bool>> {
    matrix.entry(i).or_insert_with(HashMap::new).insert(j, true);
    matrix.entry(j).or_insert_with(HashMap::new).insert(i, true);
    return matrix;
}
