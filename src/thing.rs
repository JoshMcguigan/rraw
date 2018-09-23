/// Represents a comment, account, link, message, subreddit, or award
/// id is the unique identifier for the thing
/// name is the id prefixed with an identifier for the type of thing
pub trait Thing {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
}
