use std::env;
mod scrape;
mod tweet;
use clap::Parser;
use color_eyre::Result;
use kv::Store;
// use reqwest::Method;
use tweet::tweet;
/// Main entrypoint for X bot application
#[derive(Debug, Parser)]
pub struct PipeDown {
    #[clap(env = "X_API_KEY")]
    pub api_key: String,
    #[clap(env = "X_API_SECRET")]
    pub api_secret: String,
    #[clap(env = "BEARER_TOKEN")]
    pub bearer_token: String,
    #[clap(env = "X_ACCESS_TOKEN")]
    pub x_access_token: String,
    #[clap(env = "X_ACCESS_TOKEN_SECRET")]
    pub x_access_token_secret: String,
}
impl PipeDown {
    pub fn set_envar_from_args(&self) {
        env::set_var("X_API_KEY", &self.api_key);
        env::set_var("X_API_SECRET", &self.api_secret);
        env::set_var("BEARER_TOKEN", &self.bearer_token);
        env::set_var("X_ACCESS_TOKEN", &self.x_access_token);
        env::set_var("X_ACCESS_TOKEN_SECRET", &self.x_access_token_secret);
    }
}

fn store() -> Result<Store> {
    let cfg = kv::Config::new("./tweetdb");
    let store = kv::Store::new(cfg);
    Ok(store?)
}

async fn process_tweets() -> Result<()> {
    let ids = scrape::get_twitter_ids(scrape::scrape_tweets()?);

    tracing::info!(?ids);
    let store = store()?;
    let tweets_bucket = store.bucket::<String, String>(Some("tweets"))?;

    for id in ids {
        tracing::info!("Checking tweet {}", id);
        // check if key exists
        if tweets_bucket.get(&id)?.is_none() {
            tracing::info!("Tweet {} not replied to, replying", id);
            let delay = random_t();

            tracing::info!("Delaying for {} seconds", delay);
            tokio::time::sleep(tokio::time::Duration::from_secs(delay.into())).await;

            let res = tweet(id.clone()).await;
            if res.is_ok() {
                tweets_bucket.set(&id, &"true".to_string())?;
                tweets_bucket.flush_async().await?;
            }
        } else {
            tracing::info!("Tweet {} already replied to", id);
        }
    }

    Ok(())
}

// generate random number between 0 and 59
fn random_t() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(0..59)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // set up tracing
    tracing_subscriber::fmt::fmt()
        .with_env_filter("info")
        .init();

    tracing::info!("Starting X bot");

    let args = PipeDown::parse();
    args.set_envar_from_args();
    let _ = process_tweets().await;
    // let _ = tweet("1841274170685935834".into()).await;

    let mut cron = cronjob::CronJob::new("test", |_: &str| {
        tokio::spawn(async {
            let _ = process_tweets().await;
        });
    });

    cron.minutes(&random_t().to_string());
    cron.seconds(&random_t().to_string());
    cron.start_job();

    Ok(())
}
