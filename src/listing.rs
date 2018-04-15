use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Deserialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum Thing {
    #[serde(rename = "t1")]
    Comment(Comment),
    #[serde(rename = "t3")]
    Link(Link)
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum Listing {
    Listing { children: Vec<Thing> }
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
pub struct Comment {
    pub id: String,
    pub body: String,
    #[serde(deserialize_with="parse_listing")]
    pub replies: Listing
}

fn parse_listing<'de, D>(d: D) -> Result<Listing, D::Error>
    where D: Deserializer<'de>
{
    match Deserialize::deserialize(d) {
        Ok(listing) => Ok(listing),
        _ => Ok(Listing::Listing {children: vec![]})
    }
}
