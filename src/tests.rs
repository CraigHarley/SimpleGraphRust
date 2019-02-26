use time::PreciseTime;

#[test]
fn big_blob_of_tests() {
    let start = PreciseTime::now();
    let matrix = create_graph_from_mysql();
    let end = PreciseTime::now();
    println!("{} seconds to start up.", start.to(end));

    let from_ids = [2266, 17, 17, 9682, 3405];
    let to_ids = [3002, 3002, 15031, 14658, 2773];

    for (i, _y) in from_ids.iter().enumerate() {
        let start: PreciseTime = PreciseTime::now();
        let result: Result = bfs(&matrix, from_ids[i], to_ids[i]);
        println!("Visited: {} Path: {:?} Success: {}", result.visited_count, result.path, result.success);
        let end: PreciseTime = PreciseTime::now();
        println!("{} seconds to run...", start.to(end));
        println!();
    }
}