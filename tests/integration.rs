extern crate rraw;
extern crate dotenv;

use std::time::SystemTime;
use rraw::RRAWResult;
use std::thread;
use std::time;

// NOTE: new accounts are only allowed to post once per 10 minutes, so these tests could fail if run repeatedly

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
fn test() -> RRAWResult<()> {
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
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let post_title = format!("Testing RRAW - {:?}", timestamp);
    let post_body = format!("Post body - {:?}", timestamp);

    let response = reddit_client.submit(subreddit, "self", &post_title, &post_body)?;
    let mut links = reddit_client.new(subreddit, 1)?;

    let test_post = links.pop().unwrap();

    assert_eq!(test_post.title, post_title);

    let comment_text = "This is a test comment.";
    reddit_client.reply(&response.name, comment_text)?;

    // todo add testing of comments route
    //    let comments = reddit_client.comments(subreddit, &response.id)?;
    //
    //    assert_eq!(1, comments.len(), "Expected comment to be posted and returned");
    //    assert_eq!(comment_text, comments[0].body);

    Ok(())
}
