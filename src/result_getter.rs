use crate::graph::SearchResult;
use crate::pool::get_pool;
use mysql::QueryResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct FormattedResult {
    links: Vec<LinkDetails>,
}

pub fn result_getter(search_result: SearchResult) -> FormattedResult {
    let mut formatted_result = FormattedResult { links: vec![] };

    if search_result.success {
        let mut current = 0;

        while (current + 1) < search_result.path.len() {
            formatted_result.links.push(get_link_details(
                search_result.path[current],
                search_result.path[current + 1],
            ));
            current = current + 1;
        }
    }

    return formatted_result;
}

#[derive(Serialize)]
struct LinkDetails {
    team_name: String,
    years: String,
    from_player_name: String,
    to_player_name: String,
}

fn get_link_details(first: u32, second: u32) -> LinkDetails {
    let mut link_details: Vec<LinkDetails> = get_pool()
        .prep_exec(
            "
        SELECT    t.name as team_name,
                  pl.years as years,
                  fromPlayer.name as from_player_name,
                  toPlayer.name as to_player_name
        FROM      playerlinks AS pl
        LEFT JOIN team t              ON t.id = pl.teamID
        LEFT JOIN player fromPlayer   ON fromPlayer.id = pl.fromPlayerID
        LEFT JOIN player toPlayer     ON toPlayer.id = pl.toPlayerID
        WHERE     fromPlayerId = ? 
        AND       toPlayerId   = ? 
        LIMIT     1
        ",
            (first, second),
        )
        .map(|result: QueryResult| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (team_name, years, from_player_name, to_player_name) = mysql::from_row(row);
                    LinkDetails {
                        team_name,
                        years,
                        from_player_name,
                        to_player_name,
                    }
                })
                .collect()
        })
        .unwrap();

    link_details.pop().unwrap()
}
