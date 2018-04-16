use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Deserialize, Debug)]
pub struct Container<T> {
    pub data: T
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub url: String,
    pub id: String,
    pub title: String,
    pub subreddit: String,
    pub num_comments: u32
}

#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    pub after: Option<String>,
    pub children: Vec<T>
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    pub body: String,
    #[serde(deserialize_with="parse_listing")]
    pub replies: Option<Container<Listing<Container<Comment>>>>
}

fn parse_listing<'de, D>(d: D) -> Result<Option<Container<Listing<Container<Comment>>>>, D::Error>
    where D: Deserializer<'de>
{
    match Deserialize::deserialize(d) {
        Ok(listing) => Ok(Some(listing)),
        Err(e) => {
            Ok(None)
        }
    }
}
