use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub struct War {
    pub state: String,
    pub team_size: Option<i32>,
    pub attacks_per_member: Option<i8>,
    pub preparation_start_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub clan: WarClan,
    pub opponent: WarClan,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueGroup {
    pub state: String,
    pub season: String,
    pub clans: Vec<LeagueGroupClan>,
    pub rounds: Vec<Round>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub war_tags: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default = "default_league_group_clan")]
pub struct LeagueGroupClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: BadgeUrls,
    pub clan_level: i8,
    pub members: Vec<LeagueMember>,
}

fn default_league_group_clan() -> LeagueGroupClan {
    LeagueGroupClan {
        tag: "".to_string(),
        name: "".to_string(),
        badge_urls: BadgeUrls {
            small: "".to_string(),
            large: "".to_string(),
            medium: "".to_string(),
        },
        clan_level: 0,
        members: vec![],
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueMember {
    pub tag: String,
    pub name: String,
    pub town_hall_level: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default = "default_clan")]
pub struct WarClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: BadgeUrls,
    pub clan_level: i8,
    pub attacks: Option<i32>,
    pub stars: Option<i32>,
    pub destruction_percentage: Option<f64>,
    pub members: Option<Vec<Member>>,
}
fn default_clan() -> WarClan {
    WarClan {
        tag: "".to_string(),
        name: "".to_string(),
        badge_urls: BadgeUrls {
            small: "".to_string(),
            large: "".to_string(),
            medium: "".to_string(),
        },
        clan_level: 0,
        attacks: None,
        stars: None,
        destruction_percentage: None,
        members: None,
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub tag: String,
    pub name: String,
    pub townhall_level: i8,
    pub map_position: i32,
    pub attacks: Option<Vec<Attack>>,
    pub opponent_attacks: i32,
    pub best_opponent_attack: Option<Attack>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attack {
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: i32,
    pub destruction_percentage: f32,
    pub order: i32,
    pub duration: i32,
}

impl War {
    /// Returns the start time of this [`War`].
    ///
    /// # Panics
    ///
    /// Panics if parsing the start time fails, which should never happen.
    #[must_use]
    pub fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.start_time.as_ref().map(|start_time| {
            chrono::Utc.from_utc_datetime(
                &chrono::NaiveDateTime::parse_from_str(start_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
            )
        })
    }

    /// Returns the end time of this [`War`].
    ///
    /// # Panics
    ///
    /// Panics if parsing the end time fails, which should never happen.
    #[must_use]
    pub fn end_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.end_time.as_ref().map(|end_time| {
            chrono::Utc.from_utc_datetime(
                &chrono::NaiveDateTime::parse_from_str(end_time, "%Y%m%dT%H%M%S.%fZ").unwrap(),
            )
        })
    }

    /// Returns the preparation start time of this [`War`].
    ///
    /// # Panics
    ///
    /// Panics if parsing the preparation start time fails, which should never happen.
    #[must_use]
    pub fn preparation_start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.preparation_start_time
            .as_ref()
            .map(|preparation_start_time| {
                chrono::Utc.from_utc_datetime(
                    &chrono::NaiveDateTime::parse_from_str(
                        preparation_start_time,
                        "%Y%m%dT%H%M%S.%fZ",
                    )
                    .unwrap(),
                )
            })
    }
}
