use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Checklist {
    pub id: u32,
    pub name: String,
    pub options: i32,
    pub public: bool,
    #[serde(rename = "markdown?")]
    pub markdown: bool,
    pub archived: bool,
    pub read_only: bool,
    pub user_count: i32,
    pub percent_completed: f32,
    pub task_count: i32,
    pub task_completed: i32,
    pub item_count: i32,
    pub tags: HashMap<String, bool>,
    pub tags_as_text: String,
    #[serde(with = "checkvist_date")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "checkvist_date")]
    pub user_updated_at: DateTime<Utc>,
}

mod checkvist_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y/%m/%d %H:%M:%S %z";

    pub fn _serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn get_checklists(login: String, api_key: String) -> Result<Vec<Checklist>, std::io::Error> {
    let json = ureq::get("https://checkvist.com/checklists.json")
        .auth(&login, &api_key)
        .call()
        .into_json()?;
    let items = serde_json::from_value::<Vec<Checklist>>(json)?;
    Ok(items)
}
