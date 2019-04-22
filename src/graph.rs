use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::rc::Rc;
use mysql::QueryResult;
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct SearchResult {
    success: bool,
    path: Vec<u32>,
    visited_count: u32,
}

struct PlayerLink {
    id: u32,
    from_player_id: u32,
    to_player_id: u32,
}

pub struct Graph {
    pub matrix: HashMap<u32, HashMap<u32, u32>>,
}

#[derive(Serialize)]
struct GraphNode {
    value: u32,
    parent: Option<Rc<GraphNode>>,
}

impl fmt::Debug for GraphNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn create_graph_from_mysql() -> HashMap<u32, HashMap<u32, u32>> {
//  todo env
    let pool = mysql::Pool::new("mysql://root@localhost:3306/mysql").unwrap();
    let mut matrix = HashMap::new();

    let player_links: Vec<PlayerLink> =
        pool.prep_exec("SELECT id, fromPlayerId as from_player_id, toPlayerId as to_player_id FROM sixdegrees.playerlinks", ())
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
        matrix = add_edge(
            matrix,
            link.from_player_id,
            link.to_player_id,
            link.id,
        );
    }

    return matrix;
}

pub fn add_edge(
    mut matrix: HashMap<u32, HashMap<u32, u32>>,
    from_id: u32,
    to_id: u32,
    link_id: u32,
) -> HashMap<u32, HashMap<u32, u32>> {
    matrix
        .entry(from_id)
        .or_insert_with(HashMap::new)
        .insert(to_id, link_id);
    matrix
        .entry(to_id)
        .or_insert_with(HashMap::new)
        .insert(from_id, link_id);

    return matrix;
}

pub fn bfs(matrix: &HashMap<u32, HashMap<u32, u32>>, start_value: u32, goal_value: u32) -> SearchResult {
    if start_value == goal_value
        || !matrix.contains_key(&start_value)
        || !matrix.contains_key(&goal_value)
    {
        return SearchResult {
            success: start_value == goal_value,
            path: vec![],
            visited_count: 0,
        };
    }

    let mut queue: VecDeque<Rc<GraphNode>> = VecDeque::new();
    let mut visited_nodes: Vec<u32> = vec![];
    for value in get_unvisited_neighbors(&matrix, &visited_nodes, start_value) {
        queue.push_front(Rc::new(GraphNode {
            value,
            parent: Option::None,
        }));
    }

    let mut visited_count: u32 = 0;
    loop {
        visited_count = visited_count + 1;

        match queue.pop_back() {
            Some(current_node) => {
                if current_node.value == goal_value {
                    let mut path: Vec<Rc<GraphNode>> = vec![];

                    let mut parent_node = Option::Some(current_node.clone());
                    loop {
                        match parent_node {
                            Some(next_parent_node) => {
                                path.push(next_parent_node.clone());
                                parent_node = next_parent_node.parent.clone();
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    path.push(
                        Rc::new(
                            GraphNode {
                                value: start_value,
                                parent: None,
                            }
                        )
                    );

                    return SearchResult {
                        success: true,
                        path: path
                            .iter()
                            .map(|f| f.value)
                            .rev()
                            .collect(),
                        visited_count,
                    };
                }
                if !visited_nodes.contains(&current_node.value) {
                    for value in
                        get_unvisited_neighbors(&matrix, &visited_nodes, current_node.value)
                        {
                            queue.push_front(Rc::new(GraphNode {
                                value,
                                parent: Option::Some(current_node.clone()),
                            }));
                        }

                    visited_nodes.push(current_node.value);
                }
            }
            _ => {
                return SearchResult {
                    success: false,
                    path: vec![],
                    visited_count,
                };
            }
        }
    }
}

pub fn get_unvisited_neighbors(
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
