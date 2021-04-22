use async_std::task;
use seahorse::{App, Context, Flag, FlagType};
use serde::Deserialize;
use std::env;
use std::process::exit;
use surf::http::Url;

#[derive(Deserialize, Debug)]
struct Translation {
    detected_source_language: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    translations: Vec<Translation>,
}

async fn translate<'a>(auth_key: &'a str, text: &'a str, target_lang: &'a str) -> surf::Result<()> {
    let url = Url::parse(&format!(
        "https://api-free.deepl.com/v2/translate?auth_key={}&text={}&target_lang={}",
        auth_key, text, target_lang
    ))?;

    let res: Response = match surf::get(url).recv_string().await {
        Ok(res) => match serde_json::from_str(&res) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    for translation in res.translations {
        println!("{}", translation.text)
    }

    Ok(())
}

fn action(c: &Context) {
    task::block_on(async {
        let auth_key = match c.string_flag("key") {
            Ok(key) => key,
            Err(_) => match env::var("DEEPL_KEY") {
                Ok(key) => key,
                Err(_) => {
                    c.help();
                    exit(1);
                }
            },
        };

        let text = if c.args.is_empty() {
            c.help();
            exit(1);
        } else {
            c.args.join(" ")
        };

        let lang = match c.string_flag("lang") {
            Ok(lang) => lang,
            Err(_) => "EN".to_string(),
        };

        translate(&auth_key, &text, &lang).await.unwrap();
    });
}

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [text]", env!("CARGO_PKG_NAME")))
        .action(action)
        .flag(
            Flag::new("key", FlagType::String)
                .alias("k")
                .description("Auth key"),
        )
        .flag(
            Flag::new("lang", FlagType::String)
                .alias("l")
                .description("Target language(EN, JA, DE, etc...)"),
        )
        .run(env::args().collect())
}
