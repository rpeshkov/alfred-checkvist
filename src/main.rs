#[macro_use]
extern crate serde;

use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
struct Checklist {
    id: u32,
    name: String,
    options: i32,
    public: bool,
    #[serde(rename = "markdown?")]
    markdown: bool,
    archived: bool,
    read_only: bool,
    user_count: i32,
    percent_completed: f32,
    task_count: i32,
    task_completed: i32,
    item_count: i32,
    tags: HashMap<String, bool>,
    tags_as_text: String,
    #[serde(with = "checkvist_date")]
    updated_at: DateTime<Utc>,
    #[serde(with = "checkvist_date")]
    user_updated_at: DateTime<Utc>,
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

fn get_checklists(login: String, api_key: String) -> Result<Vec<Checklist>, std::io::Error> {
    let json = ureq::get("https://checkvist.com/checklists.json")
        .auth(&login, &api_key)
        .call()
        .into_json()?;
    let items = serde_json::from_value::<Vec<Checklist>>(json)?;
    Ok(items)
}

fn main() -> Result<(), std::io::Error> {
    let login = std::env::var("cv_login").unwrap_or_default();
    let api_key = std::env::var("cv_apikey").unwrap_or_default();

    match get_checklists(login, api_key) {
        Ok(checklists) => {
            let items = checklists
                .into_iter()
                .map(|x| {
                    alfred::ItemBuilder::new(x.name)
                        .arg(x.id.to_string())
                        .subtitle(x.tags_as_text)
                        .into_item()
                })
                .collect::<Vec<_>>();

            alfred::json::write_items(std::io::stdout(), &items)
        }
        Err(_) => {
            let items = vec![
                alfred::ItemBuilder::new("Checklists fetch has failed!")
                    .subtitle("Please check your credentials in the Workflow's settings")
                    .icon_path("/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertStopIcon.icns")
                    .into_item(),
            ];
            alfred::json::write_items(std::io::stdout(), &items)
        }
    }
}
