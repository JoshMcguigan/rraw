extern crate rraw;
extern crate serde_json;
extern crate dotenv;

use rraw::authorize;
use rraw::new;
use rraw::comments;

fn main() {
    let reddit_user_agent = dotenv::var("REDDIT_USER_AGENT").unwrap();
    let reddit_username = dotenv::var("REDDIT_USERNAME").unwrap();
    let reddit_password = dotenv::var("REDDIT_PASSWORD").unwrap();
    let reddit_client_id = dotenv::var("REDDIT_CLIENT_ID").unwrap();
    let reddit_client_secret = dotenv::var("REDDIT_CLIENT_SECRET").unwrap();

    match authorize(&reddit_username, &reddit_password, &reddit_client_id, &reddit_client_secret, &reddit_user_agent) {
        Ok(auth_data) => {
            let subreddit = "test";
            match new(&auth_data.access_token, &reddit_user_agent, subreddit, 2) {
                Ok(links) => {
                    for link in links.iter() {
                        println!("{:?}", link.title);
                        println!("{:#?}", comments(&auth_data.access_token, &reddit_user_agent, subreddit, &link.id))
                    }
                },
                Err(e) => println!("error = {:?}", e)
            };
        },
        Err(e) => println!("error = {:?}", e)
    };
}
