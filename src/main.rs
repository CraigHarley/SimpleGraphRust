use std::collections::HashMap;

fn main() {
    let matrix = HashMap::new();
    let step1 = add_edge(matrix, 1, 2);
    let step2 = add_edge(step1, 5, 1);
    let step3 = add_edge(step2, 5, 2);
    let step4 = add_edge(step3, 2, 6);

    for (&i, second_level) in step4.iter() {
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
