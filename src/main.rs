#[macro_use]
extern crate serde;

mod checkvist;

const LOGIN_VAR: &str = "cv_login";
const API_KEY_VAR: &str = "cv_apikey";

const ERROR_ICON: &str =
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertStopIcon.icns";
const FETCH_FAILED_MESSAGE: &str = "Checklists fetch has failed!";
const FETCH_FAILED_DESCRIPTION: &str = "Please check your credentials in the Workflow's settings";

fn alfred_error(title: &str, subtitle: &str) {
    let items = vec![
        alfred::ItemBuilder::new(title)
            .subtitle(subtitle)
            .icon_path(ERROR_ICON)
            .into_item(),
    ];
    alfred::json::write_items(std::io::stdout(), &items).unwrap();
}

fn main() -> Result<(), std::io::Error> {
    let login = match std::env::var(LOGIN_VAR) {
        Ok(v) => v,
        Err(e) => {
            alfred_error("No variable 'cv_login'!",
                         "Please define the variable in the workflow settings");
            panic!(e);
        }
    };

    let api_key = match std::env::var(API_KEY_VAR) {
        Ok(v) => v,
        Err(e) => {
            alfred_error("No variable 'cv_apikey'",
                         "Please define the variable in the workflow settings");
            panic!(e);
        }
    };

    let args = std::env::args().collect::<Vec<String>>();

    match checkvist::get_checklists(login, api_key) {
        Ok(checklists) => {
            let items = checklists
                .into_iter()
                .filter(|x| {
                    if args.len() > 1 {
                        x.name.to_lowercase().contains(&args[1].to_lowercase())
                    } else {
                        true
                    }
                })
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
            alfred_error(FETCH_FAILED_MESSAGE, FETCH_FAILED_DESCRIPTION);
            panic!(FETCH_FAILED_MESSAGE);
        }
    }
}
