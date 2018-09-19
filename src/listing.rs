use serde::de::Deserializer;
use serde::Deserialize;

extern crate serde;

#[derive(Deserialize, Debug)]
pub struct Container<T> {
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub url: String,
    pub id: String,
    pub name: String, // full unique identifier
    pub title: String,
    pub subreddit: String,
    pub num_comments: u32,
    pub created_utc: f64,
}

#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    pub after: Option<String>,
    pub children: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct CommentFullRepliesStructure {
    pub id: String,
    pub body: String,
    #[serde(deserialize_with = "parse_listing")]
    pub replies: Option<Container<Listing<Container<CommentFullRepliesStructure>>>>,
}

#[derive(Debug)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub replies: Vec<Comment>,
}

fn parse_listing<'de, D>(
    d: D,
) -> Result<Option<Container<Listing<Container<CommentFullRepliesStructure>>>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(d) {
        Ok(listing) => Ok(Some(listing)),
        Err(_) => Ok(None),
    }
}

// TODO add unit testing here
