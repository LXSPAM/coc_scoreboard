use serde::Serialize;

use crate::error::APIError::*;
use crate::login::get_client;
use crate::war::War;
use crate::Response;

#[tauri::command]
pub async fn get_war(tag: String) -> Response {
    match get_client().await {
        Ok(client) => match client.get_current_war(&tag, false).await {
            Ok(clan) => Response {
                status: 200,
                message: serde_json::to_string_pretty(&clan).unwrap(),
            },
            Err(e) => match e {
                ClientNotReady => Response {
                    status: 400,
                    message: "Client is not ready.".to_string(),
                },
                BadParameters => Response {
                    status: 400,
                    message: "Bad parameters.".to_string(),
                },
                AccessDenied => Response {
                    status: 403,
                    message: "Access denied.".to_string(),
                },
                NotFound => Response {
                    status: 404,
                    message: "Not found.".to_string(),
                },
                RequestThrottled => Response {
                    status: 429,
                    message: "Request throttled.".to_string(),
                },
                UnknownError => Response {
                    status: 500,
                    message: "Unknown error.".to_string(),
                },
                InMaintenance => Response {
                    status: 503,
                    message: "In maintenance.".to_string(),
                },
                _ => Response {
                    status: 404,
                    message: e.to_string(),
                },
            },
        },
        Err(_) => Response {
            status: 400,
            message: "Client is not initialized.".to_string(),
        },
    }
}

#[derive(Clone, Serialize)]
pub struct WarData {
    clan: ClanData,
    opponent: ClanData,
}

#[derive(Clone, Serialize)]
struct ClanData {
    badge: String,
    stars: String,
    percentage: String,
    duration: String,
    attacks: String,
}

#[tauri::command]
pub fn parse_wardata(data: String) -> Result<Response, String> {
    let war: War = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let clan_duration = format_duration(calculate_average_duration(
        war.clan.members.as_ref().unwrap(),
    ));
    let opponent_duration = format_duration(calculate_average_duration(
        war.opponent.members.as_ref().unwrap(),
    ));

    fn format_percentage(value: f64) -> String {
        if value == 100.0 {
            format!("{:.0}", value)
        } else {
            format!("{:.1}", value)
        }
    }

    let clan = ClanData {
        badge: war.clan.badge_urls.large.clone(),
        stars: war.clan.stars.unwrap_or(0).to_string(),
        percentage: format_percentage(war.clan.destruction_percentage.unwrap_or(0.0)),
        duration: clan_duration,
        attacks: format!(
            "{}/{}",
            war.clan.attacks.unwrap_or(0),
            war.attacks_per_member.unwrap() as i32
                * war.clan.members.as_ref().unwrap().len() as i32
        ),
    };

    let opponent = ClanData {
        badge: war.opponent.badge_urls.large.clone(),
        stars: war.opponent.stars.unwrap_or(0).to_string(),
        percentage: format_percentage(war.opponent.destruction_percentage.unwrap_or(0.0)),
        duration: opponent_duration,
        attacks: format!(
            "{}/{}",
            war.opponent.attacks.unwrap_or(0),
            war.attacks_per_member.unwrap() as i32
                * war.opponent.members.as_ref().unwrap().len() as i32
        ),
    };

    let war_data = WarData { clan, opponent };
    let response = Response {
        status: 200,
        message: serde_json::to_string_pretty(&war_data).unwrap(),
    };
    Ok(response)
}

fn calculate_average_duration(members: &Vec<crate::war::Member>) -> u32 {
    let mut total = 0;
    let mut count = 0;
    for member in members {
        if let Some(attacks) = &member.attacks {
            for attack in attacks {
                total += attack.duration;
                count += 1;
            }
        }
    }
    if count > 0 {
        (total / count) as u32
    } else {
        0
    }
}

fn format_duration(seconds: u32) -> String {
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{}:{:02}", minutes, remaining_seconds)
}
