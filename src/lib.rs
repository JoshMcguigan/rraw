extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod listing;
use listing::Listing;
use listing::Link;

use std::collections::HashMap;
use reqwest::header::{UserAgent, ContentType, Authorization, Bearer};
use listing::Container;
use listing::CommentFullRepliesStructure;
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

pub fn authorize(reddit_username: &str, reddit_password: &str, reddit_client_id: &str, reddit_client_secret: &str, reddit_user_agent: &str) -> Result<AuthData, Error> {
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("username", &reddit_username);
    map.insert("password", &reddit_password);

    let client = reqwest::Client::new();
    match client.post("https://www.reddit.com/api/v1/access_token")
        .header(UserAgent::new(reddit_user_agent.to_owned()))
        .header(ContentType::form_url_encoded())
        .basic_auth(reddit_client_id, Some(reddit_client_secret))
        .form(&map)
        .send()?
        .json() {
        Ok(auth_data) => Ok(auth_data),
        Err(e) => Err(e.into())
    }
}

pub fn new(token: &str, reddit_user_agent: &str, subreddit: &str, limit: usize) -> Result<Vec<Link>, Error> {
    let client = reqwest::Client::new();
    let result : Result<Container<Listing<Container<Link>>>, reqwest::Error> = client.get(&("https://oauth.reddit.com/r/".to_owned() + subreddit + "/new?limit="+&limit.to_string()))
        .header(UserAgent::new(reddit_user_agent.to_owned()))
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

fn format_comments(comments: Option<Container<Listing<Container<CommentFullRepliesStructure>>>>) -> Vec<Comment> {
    match comments {
        Some(comments) => {
            comments.data.children.into_iter().map(
                |comment_container|
                    Comment {id: comment_container.data.id, body: comment_container.data.body, replies: format_comments(comment_container.data.replies)}
            ).collect()
        },
        None => vec![]
    }

}

pub fn comments(token: &str, reddit_user_agent: &str, subreddit: &str, id: &str) -> Result<Vec<Comment>, Error> {
    let client = reqwest::Client::new();
    let result : Result<serde_json::Value, reqwest::Error> = client.get(&("https://oauth.reddit.com/r/".to_owned() + subreddit + "/comments/" + id + "?depth=100000&limit=1000000&showmore=false"))
        .header(UserAgent::new(reddit_user_agent.to_owned()))
        .header(Authorization(
            Bearer {
                token: token.to_owned()
            }
        ))
        .send()?
        .json();

    match result {
        Ok(value) => {
            let comments : Option<Container<Listing<Container<CommentFullRepliesStructure>>>> = Some(serde_json::from_value(value[1].clone())?);
            Ok(format_comments(comments))
        },
        Err(e) => Err(Error::Network(e))
    }
}

    pub fn reply(token: &str, reddit_user_agent: &str, parent_id: &str, body: &str) {
        let client = reqwest::Client::new();
        let params = [("thing_id", parent_id), ("text", body)];
        let url = "https://oauth.reddit.com/api/comment";
        let res = client.post(url)
            .header(UserAgent::new(reddit_user_agent.to_owned()))
            .header(Authorization(
                Bearer {
                    token: token.to_owned()
                }
            ))
            .header(ContentType::form_url_encoded())
            .form(&params)
            .send();

        println!("{:#?}", res);

    }
