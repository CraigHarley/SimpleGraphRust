extern crate mysql;

use std::collections::HashMap;
use mysql::QueryResult;

fn main() {
    insert_edges_from_mysql();

    let matrix = create_graph();

    for (&i, second_level) in matrix.iter() {
        for (&j, &value) in second_level.iter() {
            println!("Calling {}, {}: {}", i, j, value);
        }
    }
}

fn create_graph() -> HashMap<u32, HashMap<u32, bool>> {
    let matrix = HashMap::new();
    let matrix = add_edge(matrix, 1, 2);
    let matrix = add_edge(matrix, 5, 1);
    let matrix = add_edge(matrix, 5, 2);
    let matrix = add_edge(matrix, 2, 6);
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


struct PlayerLink {
    id: u32,
    fromPlayerId: u32,
    toPlayerId: u32,
}

fn insert_edges_from_mysql() {
    let pool = mysql::Pool::new("mysql://root@localhost:3306/mysql").unwrap();

    let player_links: Vec<PlayerLink> =
        pool.prep_exec("SELECT id, fromPlayerId, toPlayerId FROM sixdegrees.player_links LIMIT 100", ())
            .map(|result: QueryResult| {
                result.map(|x|
                    x.unwrap())
                    .map(|row| {
                        let (id, fromPlayerId, toPlayerId) = mysql::from_row(row);
                        PlayerLink {
                            id,
                            fromPlayerId,
                            toPlayerId,
                        }
                    }).collect()
            }).unwrap();

    for x in player_links {
        println!("Links: {}, {}: {}", x.id, x.fromPlayerId, x.toPlayerId);
    }
}
