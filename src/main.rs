#![feature(proc_macro_hygiene, decl_macro)]
extern crate mysql;
extern crate time;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::State;
use rocket_contrib::json::Json;

mod graph;
use crate::graph::Graph;
use crate::graph::create_graph_from_mysql;
use crate::graph::bfs;
use crate::graph::SearchResult;


fn main() {
    rocket::ignite()
        .manage(Graph {
            matrix: create_graph_from_mysql(),
        })
        .mount("/sixdegrees", routes![sixdegrees])
        .launch();
}

#[get("/<first>/<second>")]
fn sixdegrees(first: u32, second: u32, graph: State<Graph>) -> Json<SearchResult> {
    Json(bfs(&graph.matrix, first, second))
}
