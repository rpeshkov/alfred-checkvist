#[macro_use]
extern crate serde;

mod checkvist;

const ERROR_ICON: &str =
    "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertStopIcon.icns";
const FETCH_FAILED_MESSAGE: &str = "Checklists fetch has failed!";
const FETCH_FAILED_DESCRIPTION: &str = "Please check your credentials in the Workflow's settings";

fn main() -> Result<(), std::io::Error> {
    let login = std::env::var("cv_login").unwrap_or_default();
    let api_key = std::env::var("cv_apikey").unwrap_or_default();

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
            let items = vec![
                alfred::ItemBuilder::new(FETCH_FAILED_MESSAGE)
                    .subtitle(FETCH_FAILED_DESCRIPTION)
                    .icon_path(ERROR_ICON)
                    .into_item(),
            ];
            alfred::json::write_items(std::io::stdout(), &items)
        }
    }
}
