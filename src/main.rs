extern crate mysql;

use std::collections::HashMap;
use mysql::QueryResult;

struct PlayerLink {
    id: u32,
    fromPlayerId: u32,
    toPlayerId: u32,
}

fn main() {
    let matrix = create_graph_from_mysql();

    for (&i, second_level) in matrix.iter() {
        for (&j, &value) in second_level.iter() {
            println!("Calling {}, {}: {}", i, j, value);
        }
    }
}

fn create_graph_from_mysql() -> HashMap<u32, HashMap<u32, bool>> {
    let pool = mysql::Pool::new("mysql://root@localhost:3306/mysql").unwrap();
    let mut matrix = HashMap::new();

    let player_links: Vec<PlayerLink> =
        pool.prep_exec("SELECT id, fromPlayerId, toPlayerId FROM sixdegrees.player_links", ())
            .map(|result: QueryResult| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, fromPlayerId, toPlayerId) = mysql::from_row(row);
                        PlayerLink {
                            id,
                            fromPlayerId,
                            toPlayerId,
                        }
                    }).collect()
            }).unwrap();

    for link in player_links {
        matrix = add_edge(matrix, link.fromPlayerId, link.toPlayerId);
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