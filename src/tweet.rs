
use std::{env, path::PathBuf};
use twapi_v2::{
    api::post_2_tweets::Media, error::Error, oauth10a::OAuthAuthentication,
    upload::media_category::MediaCategory,
};

use color_eyre::Result;

pub fn new_oauth() -> OAuthAuthentication {
    OAuthAuthentication::new(
        &env::var("X_API_KEY").unwrap(),
        &env::var("X_API_SECRET").unwrap(),
        &env::var("X_ACCESS_TOKEN").unwrap(),
        &env::var("X_ACCESS_TOKEN_SECRET").unwrap(),
    )
}

pub async fn tweet(tweet_id: String) -> Result<()> {
    let oauth = new_oauth();

    let media_id = upload_media().await.unwrap();

    let body = twapi_v2::api::post_2_tweets::Body {
        text: None,
        reply: Some(twapi_v2::api::post_2_tweets::Reply {
            in_reply_to_tweet_id: tweet_id,
            ..Default::default()
        }),
        media: Some(Media {
            media_ids: vec![media_id.to_string()],
            ..Default::default()
        }),
        ..Default::default()
    };

    let (response, _headers) = twapi_v2::api::post_2_tweets::Api::new(body)
        .execute(&oauth)
        .await?;
    tracing::info!(?response);

    Ok(())
}

pub async fn upload_media() -> Result<u64, Error> {
    let oauth = new_oauth();
    let (upload_response, _header) = twapi_v2::upload::upload_media(
        &PathBuf::from("assets/pipedown.gif"),
        "image/gif",
        Some(MediaCategory::TweetGif),
        None,
        &oauth,
    )
    .await?;

    Ok(upload_response.media_id)
}
