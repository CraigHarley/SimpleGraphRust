extern crate mysql;
extern crate time;

use std::collections::HashMap;
use mysql::QueryResult;
use std::collections::vec_deque::VecDeque;
use time::PreciseTime;

struct PlayerLink {
    id: u32,
    from_player_id: u32,
    to_player_id: u32,
}

struct GraphNode {
    value: u32,
    is_visited: bool,
    parent: Option<Box<GraphNode>>,
}

struct Result {
    success: bool,
    path: Vec<u32>,
    visited_count: u32,
}

fn main() {
    let start = PreciseTime::now();
    let matrix = create_graph_from_mysql();
    let end = PreciseTime::now();
    println!("{} seconds to start up.", start.to(end));

//    for (&i, second_level) in matrix.iter() {
//        for (&j, &value) in second_level.iter() {
//            println!("Calling {}, {}: {}", i, j, value);
//        }
//    }

    let from_ids = [2266, 17, 17, 9682, 3405];
    let to_ids = [3002, 3002, 15031, 14658, 2773];

    for (i, _y) in from_ids.iter().enumerate() {
        let start: PreciseTime = PreciseTime::now();
        let result: Result = bfs(&matrix, from_ids[i], to_ids[i]);
        println!("Visited: {} Path: {} Success: {}", result.visited_count, "N/A", result.success);
        let end: PreciseTime = PreciseTime::now();
        println!("{} seconds to run...", start.to(end));
        println!();
    }
}

fn create_graph_from_mysql() -> HashMap<u32, HashMap<u32, u32>> {
    let pool = mysql::Pool::new("mysql://root@localhost:3306/mysql").unwrap();
    let mut matrix = HashMap::new();

    let player_links: Vec<PlayerLink> =
        pool.prep_exec("SELECT id, fromPlayerId as from_player_id, toPlayerId as to_player_id FROM sixdegrees.player_links", ())
            .map(|result: QueryResult| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, from_player_id, to_player_id) = mysql::from_row(row);
                        PlayerLink {
                            id,
                            from_player_id,
                            to_player_id,
                        }
                    }).collect()
            }).unwrap();

    for link in player_links {
        matrix = add_edge(matrix, link.from_player_id, link.to_player_id, link.id);
    }

    return matrix;
}

fn add_edge(
    mut matrix: HashMap<u32, HashMap<u32, u32>>,
    from_id: u32,
    to_id: u32,
    link_id: u32,
) -> HashMap<u32, HashMap<u32, u32>> {
    matrix.entry(from_id).or_insert_with(HashMap::new).insert(to_id, link_id);
    matrix.entry(to_id).or_insert_with(HashMap::new).insert(from_id, link_id);

    return matrix;
}

fn bfs(
    matrix: &HashMap<u32, HashMap<u32, u32>>,
    start_value: u32,
    goal_value: u32,
) -> Result {
    if start_value == goal_value || !matrix.contains_key(&start_value) || !matrix.contains_key(&goal_value) {
        return Result {
            success: start_value == goal_value,
            path: vec![],
            visited_count: 0,
        };
    }

    let mut queue: VecDeque<GraphNode> = VecDeque::new();
    let mut visited_nodes: Vec<u32> = vec![];
    for value in get_unvisited_neighbors(&matrix, &visited_nodes, start_value) {
        queue.push_front(
            GraphNode {
                value,
                is_visited: true,
                parent: None,
            }
        );
    };

    let mut visited_count: u32 = 0;
    loop {
        visited_count = visited_count + 1;

        match queue.pop_back() {
            Some(current_node) => {
//              println!("checking: {} == {}", current_node, goal_node);
                if current_node.value == goal_value {


//                  todo unwind path here

                    return Result {
                        success: true,
                        path: vec![],
                        visited_count,
                    };
                }

                if !visited_nodes.contains(&current_node.value) {
                    for value in get_unvisited_neighbors(&matrix, &visited_nodes, current_node.value) {
                        queue.push_front(
                            GraphNode {
                                value,
                                is_visited: true,
                                parent: None,
                            }
                        );
                    };
                    visited_nodes.push(current_node.value);
                }
            }
            _ => {
                return Result {
                    success: false,
                    path: vec![],
                    visited_count,
                };
            }
        }
    }
}

fn get_unvisited_neighbors(
    matrix: &HashMap<u32, HashMap<u32, u32>>,
    visited_nodes: &Vec<u32>,
    i: u32,
) -> Vec<u32> {
    if matrix.contains_key(&i) {
        return matrix[&i]
            .keys()
            .filter(|&f| !visited_nodes.contains(f))
            .map(|f| *f)
            .collect();
    }

    return vec![];
}
