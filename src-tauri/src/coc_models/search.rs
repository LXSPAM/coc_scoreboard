use crate::login::CLIENT;
use crate::Response;
use levenshtein::levenshtein;

#[tauri::command]
pub async fn search_clans(query: String) -> Result<Response, Response> {
    let client = {
        let client_guard = CLIENT.get().unwrap().lock();
        match client_guard {
            Ok(guard) => guard.clone(),
            Err(_) => {
                return Err(Response {
                    status: 400,
                    message: "Client is poisoned.".to_string(),
                });
            }
        }
    };
    if let Some(client) = client {
        let clan_search = crate::clan_search::ClanSearchOptionsBuilder::new()
            .name(query.clone())
            .build();
        match client.get_clans(clan_search).await {
            Ok(clans) => {
                let query_lowercase = query.to_lowercase();
                let mut sorted_clans = clans.items.iter().collect::<Vec<_>>();

                sorted_clans.sort_by(|a, b| {
                    let a_name_lower = a.name.to_lowercase();
                    let b_name_lower = b.name.to_lowercase();

                    let a_lev = levenshtein(&query_lowercase, &a_name_lower);
                    let b_lev = levenshtein(&query_lowercase, &b_name_lower);

                    if a_name_lower == query_lowercase && b_name_lower != query_lowercase {
                        std::cmp::Ordering::Less
                    } else if a_name_lower != query_lowercase && b_name_lower == query_lowercase {
                        std::cmp::Ordering::Greater
                    } else if a_lev != b_lev {
                        a_lev.cmp(&b_lev)
                    } else {
                        b.clan_level.cmp(&a.clan_level)
                    }
                });

                return Ok(Response {
                    status: 200,
                    message: serde_json::to_string(&sorted_clans).unwrap(),
                });
            }
            Err(e) => {
                return Err(Response {
                    status: 400,
                    message: e.to_string(),
                });
            }
        }
    } else {
        return Err(Response {
            status: 400,
            message: "Client is not initialized.".to_string(),
        });
    }
}
