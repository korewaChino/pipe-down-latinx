const USER: &str = "hayasaka_aryan";
use color_eyre::Result;
use serde_json::Value;
use std::io::BufRead;

pub fn scrape_tweets() -> Result<Vec<Value>> {
    // Let's use twscrape to scrape tweets
    tracing::info!("Scraping tweets");

    let cmd = std::process::Command::new("twscrape")
        .arg("search")
        .arg(format!("from:{}", USER))
        .arg("--limit")
        .arg("1")
        .output();

    let binding = cmd?;
    let lines = binding.stdout.lines().collect::<Result<Vec<String>, _>>()?;

    let jsons = lines
        .iter()
        .map(|line| {
            let v: Value = serde_json::from_str(line).unwrap();
            v
        })
        .collect::<Vec<Value>>();

    Ok(jsons)
}

pub fn get_twitter_ids(tweets: Vec<Value>) -> Vec<String> {
    tweets
        .iter()
        .map(|tweet| tweet["id_str"].as_str().unwrap().to_string())
        .collect()
}
