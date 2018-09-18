extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod listing;
use listing::Link;
use listing::Listing;

use listing::Comment;
use listing::CommentFullRepliesStructure;
use listing::Container;
use reqwest::header::{Authorization, Bearer, ContentType, UserAgent};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Parse(serde_json::Error),
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
    pub scope: String,
}

pub struct Client {
    user_agent: String,
    auth_data: AuthData,
}

impl Client {

    pub fn try_new(
        reddit_username: &str,
        reddit_password: &str,
        reddit_client_id: &str,
        reddit_client_secret: &str,
        reddit_user_agent: &str,
    ) -> Result<Self, Error> {
        match authorize(
            reddit_username,
            reddit_password,
            reddit_client_id,
            reddit_client_secret,
            reddit_user_agent
        ) {
            Ok(auth_data) => Ok(Client{ user_agent: reddit_user_agent.to_owned(), auth_data }),
            Err(e) => Err(e.into())
        }
    }

    pub fn new(
        &self,
        subreddit: &str,
        limit: usize,
    ) -> Result<Vec<Link>, Error> {
        let client = reqwest::Client::new();
        let container: Container<Listing<Container<Link>>> = client
            .get(&format!("https://oauth.reddit.com/r/{}/new?limit={}", subreddit, limit))
            .header(UserAgent::new(self.user_agent.clone()))
            .header(Authorization(Bearer {
                token: self.auth_data.access_token.clone(),
            }))
            .send()?
            .json()?;

        Ok(container
            .data
            .children
            .into_iter()
            .map(|link_container: Container<Link>| link_container.data)
            .collect())
    }

    pub fn comments(
        &self,
        subreddit: &str,
        id: &str,
    ) -> Result<Vec<Comment>, Error> {
        let client = reqwest::Client::new();
        let json: serde_json::Value = client
            .get(
                &format!("https://oauth.reddit.com/r/{}/comments/{}?depth=100000&limit=1000000&showmore=false", subreddit, id))
            .header(UserAgent::new(self.user_agent.clone()))
            .header(Authorization(Bearer {
                token: self.auth_data.access_token.clone(),
            })).send()?
            .json()?;

        let comments: Option<Container<Listing<Container<CommentFullRepliesStructure>>>> =
            Some(serde_json::from_value(json[1].clone())?);

        Ok(format_comments(comments))
    }

    pub fn reply(&self, parent_id: &str, body: &str) {
        let client = reqwest::Client::new();
        let params = [("thing_id", parent_id), ("text", body)];
        let url = "https://oauth.reddit.com/api/comment";
        let _res = client
            .post(url)
            .header(UserAgent::new(self.user_agent.clone()))
            .header(Authorization(Bearer {
                token: self.auth_data.access_token.clone(),
            })).header(ContentType::form_url_encoded())
            .form(&params)
            .send();

        // todo return result here
    }
}

fn authorize(
    reddit_username: &str,
    reddit_password: &str,
    reddit_client_id: &str,
    reddit_client_secret: &str,
    reddit_user_agent: &str,
) -> Result<AuthData, Error> {
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("username", &reddit_username);
    map.insert("password", &reddit_password);

    let client = reqwest::Client::new();
    match client
        .post("https://www.reddit.com/api/v1/access_token")
        .header(UserAgent::new(reddit_user_agent.to_owned()))
        .header(ContentType::form_url_encoded())
        .basic_auth(reddit_client_id, Some(reddit_client_secret))
        .form(&map)
        .send()?
        .json()
    {
        Ok(auth_data) => Ok(auth_data),
        Err(e) => Err(e.into()),
    }
}

fn format_comments(
    comments: Option<Container<Listing<Container<CommentFullRepliesStructure>>>>,
) -> Vec<Comment> {
    match comments {
        Some(comments) => comments
            .data
            .children
            .into_iter()
            .map(|comment_container| Comment {
                id: comment_container.data.id,
                body: comment_container.data.body,
                replies: format_comments(comment_container.data.replies),
            }).collect(),
        None => vec![],
    }
}
