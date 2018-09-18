extern crate dotenv;
extern crate rraw;
extern crate serde_json;

fn main() -> Result<(), rraw::Error> {
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
    match reddit_client.new(subreddit, 2) {
        Ok(links) => {
            for link in links.iter() {
                println!("{:?}", link.title);
                println!(
                    "{:#?}",
                    reddit_client.comments(subreddit, &link.id)
                )
            }
        }
        Err(e) => println!("error = {:?}", e),
    };

    Ok(())
}
