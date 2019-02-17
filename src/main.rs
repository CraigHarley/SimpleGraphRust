extern crate mysql;

use std::collections::HashMap;
use mysql::QueryResult;
use std::collections::vec_deque::VecDeque;

struct PlayerLink {
    id: u32,
    from_player_id: u32,
    to_player_id: u32,
}

fn main() {
    let matrix = create_graph_from_mysql();

//    for (&i, second_level) in matrix.iter() {
//        for (&j, &value) in second_level.iter() {
//            println!("Calling {}, {}: {}", i, j, value);
//        }
//    }

    println!("Searching {}", bfs(&matrix, 2266, 3002));
//    println!("Searching {}", bfs(&matrix, 824, 8));
//    println!("Searching {}", bfs(&matrix, 2440, 93243));
}

fn create_graph_from_mysql() -> HashMap<u32, HashMap<u32, bool>> {
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
        matrix = add_edge(matrix, link.from_player_id, link.to_player_id);
    }

    return matrix;
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

fn bfs(matrix: &HashMap<u32, HashMap<u32, bool>>, start_node: u32, goal_node: u32) -> bool {
    if start_node == goal_node || !matrix.contains_key(&start_node) || !matrix.contains_key(&goal_node) {
        println!("got here 3, {} {} {}", start_node == goal_node, !matrix.contains_key(&start_node), !matrix.contains_key(&goal_node));

        return false;
    }

    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut visited_nodes: Vec<u32> = vec![];
    for x in get_unvisited_neighbors(&matrix, &visited_nodes, start_node) {
        queue.push_front(x);
    };

    loop {
        match queue.pop_back() {
            Some(current_node) => {
//                println!("checking: {} == {}", current_node, goal_node);
                if current_node == goal_node {
                    return true;
                }

                if !visited_nodes.contains(&current_node) {
                    for x in get_unvisited_neighbors(&matrix, &visited_nodes, current_node) {
                        queue.push_front(x);
                    };
                    visited_nodes.push(current_node);
                }
            }
            _ => {
                return false;
            }
        }
    }
}

fn get_unvisited_neighbors(matrix: &HashMap<u32, HashMap<u32, bool>>, visited_nodes: &Vec<u32>, i: u32) -> Vec<u32> {
    if matrix.contains_key(&i) {
        return matrix[&i]
            .keys()
            .filter(|&f| !visited_nodes.contains(f))
            .map(|f| *f)
            .collect();
    }

    return vec![];
}
