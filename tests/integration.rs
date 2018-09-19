extern crate rraw;
extern crate dotenv;

#[test]
fn try_new_error() {
    let reddit_client = rraw::Client::try_new(
        "1",
        "2",
        "3",
        "4",
        "5"
    );

    assert!(reddit_client.is_err(), "Should error with bad credentials");
}

#[test]
fn test() -> Result<(), rraw::Error> {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let reddit_username = dotenv::var("REDDIT_USERNAME").unwrap();
    let reddit_password = dotenv::var("REDDIT_PASSWORD").unwrap();
    let reddit_client_id = dotenv::var("REDDIT_CLIENT_ID").unwrap();
    let reddit_client_secret = dotenv::var("REDDIT_CLIENT_SECRET").unwrap();

    let reddit_client = rraw::Client::try_new(
        &reddit_username,
        &reddit_password,
        &reddit_client_id,
        &reddit_client_secret,
        &reddit_user_agent,
    )?;

    let subreddit = "test";
    reddit_client.submit(subreddit, "self", "testing rraw", "testing body");
    let links = reddit_client.new(subreddit, 2)?;
    for link in links.iter() {
        let _comments = reddit_client.comments(subreddit, &link.id)?;
    }

    Ok(())
}
