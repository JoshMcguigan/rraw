extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate serde_derive;
#[cfg(test)] #[macro_use] extern crate matches;

pub mod listing;
use listing::Link;
use listing::Listing;

mod response;
use response::Response;
use response::ResponseData;

mod error;
pub use error::RRAWResult;

use listing::Comment;
use listing::CommentFullRepliesStructure;
use listing::Container;
use reqwest::header::{Authorization, Bearer, ContentType, UserAgent};
use std::collections::HashMap;
use reqwest::RequestBuilder;
use serde::Serialize;

// todo settle on naming standard for all methods
// todo separate into files based on api organization?
// todo reduce string typing
// todo update to the latest version of reqwest
// todo make submit and reply functions return the same data type

#[derive(Deserialize, Debug)]
pub struct AuthData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

pub struct Client {
    http_client: reqwest::Client,
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
    ) -> RRAWResult<Self> {
        match authorize(
            reddit_username,
            reddit_password,
            reddit_client_id,
            reddit_client_secret,
            reddit_user_agent
        ) {
            Ok(auth_data) => Ok(
                Client {
                    http_client: reqwest::Client::new(),
                    user_agent: reddit_user_agent.to_owned(),
                    auth_data
                }
            ),
            Err(e) => Err(e.into())
        }
    }

    pub fn new(
        &self,
        subreddit: &str,
        limit: usize,
    ) -> RRAWResult<Vec<Link>> {
        let container: Container<Listing<Container<Link>>> = self
            .http_get(&format!("https://oauth.reddit.com/r/{}/new?limit={}", subreddit, limit))
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
    ) -> RRAWResult<Vec<Comment>> {
        let json: serde_json::Value = self
            .http_get(&format!("https://oauth.reddit.com/r/{}/comments/{}?depth=100000&limit=1000000&showmore=false", subreddit, id))
            .send()?
            .json()?;

        let comments: Option<Container<Listing<Container<CommentFullRepliesStructure>>>> =
            Some(serde_json::from_value(json[1].clone())?);

        Ok(format_comments(comments))
    }

    pub fn reply(&self, parent_id: &str, body: &str) -> RRAWResult<()> {
        // todo add test for this
        let params = [("thing_id", parent_id), ("text", body)];
        let url = "https://oauth.reddit.com/api/comment";
        self
            .http_post(url, &params)
            .send()?;

        Ok(())
    }

    pub fn submit(&self, subreddit: &str, kind: &str, title: &str, text: &str) -> RRAWResult<ResponseData> {
        // todo add test for this
        let params = [("sr", subreddit), ("kind", kind), ("title", title), ("text", text), ("api_type", "json")];
        let url = "https://oauth.reddit.com/api/submit";
        let res : Response = self
            .http_post(url, &params)
            .send()?
            .json()?;

        res.into()
    }

    fn http_get(&self, url: &str) -> RequestBuilder {
        let mut request_builder = self.http_client.get(url);
        request_builder
            .header(UserAgent::new(self.user_agent.clone()))
            .header(Authorization(Bearer {
                token: self.auth_data.access_token.clone(),
            }));

        request_builder
    }

    fn http_post<T: Serialize + ?Sized>(&self, url: &str, form_data: &T) -> RequestBuilder {
        let mut request_builder = self.http_client.post(url);
        request_builder
            .header(UserAgent::new(self.user_agent.clone()))
            .header(Authorization(Bearer {
                token: self.auth_data.access_token.clone(),
            }))
            .header(ContentType::form_url_encoded())
            .form(form_data);

        request_builder
    }
}

fn authorize(
    reddit_username: &str,
    reddit_password: &str,
    reddit_client_id: &str,
    reddit_client_secret: &str,
    reddit_user_agent: &str,
) -> RRAWResult<AuthData> {
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
