#![feature(proc_macro_hygiene, decl_macro)]
extern crate mysql;
extern crate time;
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate rocket_contrib;
extern crate serde;

use rocket::State;
use rocket_contrib::json::Json;

mod graph;
mod pool;
mod result_getter;

use crate::graph::bfs;
use crate::graph::create_graph_from_mysql;
use crate::graph::Graph;
use crate::result_getter::result_getter;
use crate::result_getter::FormattedResult;
use rocket_contrib::templates::Template;
use rocket::fairing::AdHoc;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::path::Path;

struct AssetsDir(String);

fn main() {
    rocket::ignite()
        .manage(Graph {
            matrix: create_graph_from_mysql(),
        })
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Assets Config", |rocket| {
            let assets_dir = rocket.config()
                .get_str("assets_dir")
                .unwrap_or("assets/")
                .to_string();

            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))
        .mount("/", routes![index])
        .mount("/assets", routes![assets])
        .mount("/sixdegrees", routes![sixdegrees])
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context = {
        // todo
    };
    Template::render("index", &context)
}

#[get("/<first>/<second>")]
fn sixdegrees(first: u32, second: u32, graph: State<Graph>) -> Json<FormattedResult> {
    Json(result_getter(bfs(&graph.matrix, first, second)))
}

#[get("/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}