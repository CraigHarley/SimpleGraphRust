use serde::Serialize;
use mysql::QueryResult;
use crate::pool::get_pool;
use crate::graph::SearchResult;

#[derive(Serialize)]
struct FormattedResult {
    links: Vec<LinkDetails>
}

pub fn result_getter(search_result: SearchResult) -> FormattedResult {
    let mut formatted_result = FormattedResult {
        links: vec![]
    };

    if search_result.success {
        let current = 0;

        if current + 1 <= search_result.path.len() {
            formatted_result.links.push(
                get_link_details(search_result.path[current], search_result.path[current + 1])
            );
        }
    }

    return formatted_result;
}

#[derive(Serialize)]
struct LinkDetails {}

fn get_link_details(first: u32, second: u32) -> LinkDetails {
    get_pool().prep_exec("\
        SELECT    t.name,
                  pl.years,
                  fromPlayer.name,
                  toPlayer.name
        FROM      playerlinks AS pl \
        LEFT JOIN team t              ON t.id = pl.teamID
        LEFT JOIN player fromPlayer   ON fromPlayer.id = pl.fromPlayerID
        LEFT JOIN player toPlayer     ON toPlayer.id = pl.toPlayerID
        WHERE     fromPlayerId = ? \
        AND       toPlayerId   = ? \
        LIMIT     1
        ", (first, second))
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
        })
        .unwrap()
        .pop()
}