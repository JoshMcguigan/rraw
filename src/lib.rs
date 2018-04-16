extern crate reqwest;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod listing;
use listing::Listing;
use listing::Link;

use std::collections::HashMap;
use reqwest::header::{UserAgent, ContentType, Authorization, Bearer};
use serde_json::Value;
use listing::Container;
use listing::Comment;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Parse(serde_json::Error)
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e)
    }
}

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
    match client.post("https://www.reddit.com/api/v1/access_token")
        .header(UserAgent::new(reddit_user_agent))
        .header(ContentType::form_url_encoded())
        .basic_auth(reddit_client_id, Some(reddit_client_secret))
        .form(&map)
        .send()?
        .json() {
        Ok(auth_data) => Ok(auth_data),
        Err(e) => Err(e.into())
    }
}

pub fn me(token: &str) -> Result<String, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let client = reqwest::Client::new();
    match client.get("https://oauth.reddit.com/api/v1/me")
        .header(UserAgent::new(reddit_user_agent))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .text() {
        Ok(me) => Ok(me),
        Err(e) => Err(e.into())
    }
}

pub fn new(token: &str, subreddit: &str) -> Result<Vec<Link>, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let client = reqwest::Client::new();
    let result : Result<Container<Listing<Container<Link>>>, reqwest::Error> = client.get(&("https://oauth.reddit.com/r/".to_owned() + subreddit + "/hot?limit=5"))
        .header(UserAgent::new(reddit_user_agent))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .json();

    match result {
        Ok(container) => {
            Ok(container.data.children.into_iter().map(|link_container : Container<Link> | link_container.data ).collect())
        },
        Err(e) => Err(Error::Network(e))
    }
}

pub fn comments(token: &str, subreddit: &str, id: &str) -> Result<Vec<Comment>, Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let client = reqwest::Client::new();
    let result : Result<serde_json::Value, reqwest::Error> = client.get(&("https://oauth.reddit.com/r/".to_owned() + subreddit + "/comments/" + id + "?depth=100000&limit=1000000&showmore=false"))
        .header(UserAgent::new(reddit_user_agent))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .json();

    match result {
        Ok(value) => {
            let comments : Vec<Container<Comment>> = serde_json::from_value(value[1]["data"]["children"].clone())?;
            Ok(comments.into_iter().map(|comment_container| comment_container.data ).collect())
        },
        Err(e) => Err(Error::Network(e))
    }
}
