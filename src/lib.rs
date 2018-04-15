extern crate reqwest;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod listing;
use listing::Listing;

use reqwest::Error;
use std::collections::HashMap;
use reqwest::header::{UserAgent, ContentType, Authorization, Bearer};

#[derive(Deserialize, Debug)]
pub struct AuthData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String
}

pub fn authorize() -> Result<AuthData, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let reddit_username = dotenv::var("REDDIT_USERNAME").unwrap();
    let reddit_password = dotenv::var("REDDIT_PASSWORD").unwrap();
    let reddit_client_id = dotenv::var("REDDIT_CLIENT_ID").unwrap();
    let reddit_client_secret = dotenv::var("REDDIT_CLIENT_SECRET").unwrap();

    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("username", &reddit_username);
    map.insert("password", &reddit_password);

    let client = reqwest::Client::new();
    client.post("https://www.reddit.com/api/v1/access_token")
        .header(UserAgent::new(reddit_user_agent))
        .header(ContentType::form_url_encoded())
        .basic_auth(reddit_client_id, Some(reddit_client_secret))
        .form(&map)
        .send()?
        .json()
}

pub fn me(token: &str) -> Result<String, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let client = reqwest::Client::new();
    client.get("https://oauth.reddit.com/api/v1/me")
        .header(UserAgent::new(reddit_user_agent))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .text()
}

pub fn new(token: &str, subreddit: &str) -> Result<Listing, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let client = reqwest::Client::new();
    client.get(&("https://oauth.reddit.com/r/".to_owned() + subreddit + "/new?limit=10"))
        .header(UserAgent::new(reddit_user_agent))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .json()
}

pub fn comments(token: &str, subreddit: &str, id: &str) -> Result<Vec<Listing>, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let client = reqwest::Client::new();
    client.get(&("https://oauth.reddit.com/r/".to_owned() + subreddit + "/comments/" + id + "?depth=100"))
        .header(UserAgent::new(reddit_user_agent))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .json()
}
